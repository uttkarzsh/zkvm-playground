use tracing::info;

pub fn main() {
    tracing_subscriber::fmt::init();

    let target_dir = "/tmp/jolt-guest-targets";
    let mut program = guest::compile_fib(target_dir);

    let shared_preprocessing = guest::preprocess_shared_fib(&mut program);

    let prover_preprocessing = guest::preprocess_prover_fib(shared_preprocessing.clone());
    let verifier_setup = prover_preprocessing.generators.to_verifier_setup();
    let verifier_preprocessing =
        guest::preprocess_verifier_fib(shared_preprocessing, verifier_setup);

    let prove_fib = guest::build_prover_fib(program, prover_preprocessing);
    let verify_fib = guest::build_verifier_fib(verifier_preprocessing);

    let (output, proof, io_device) = prove_fib(50);
    let is_valid = verify_fib(50, output, io_device.panic, proof);

    info!("output: {output}");
    info!("valid: {is_valid}");
}
