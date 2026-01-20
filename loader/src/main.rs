fn main() {
    println!("--- Mouseplay Loader para PS5 ---");
    
    // Tenta iniciar a lógica que está dentro da pasta mouseplay
    // Geralmente a função principal se chama 'run' ou 'start'
    if let Err(e) = mouseplay::run() {
        eprintln!("Erro ao iniciar a ferramenta: {}", e);
    }
}
