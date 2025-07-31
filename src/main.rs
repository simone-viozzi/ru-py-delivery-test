use clap::Parser;

/// A tiny CLI that prints Hello World
#[derive(Parser)]
struct Cli {}

fn main() {
    let _ = Cli::parse();
    println!("Hello from Rust!");
}
