fn main() {
    let size = 20;
    let start = Board::starting_board(size);  
    let mut board = start;
    for generation in 1..=20 {
        println!("Generation {}:", generation);
        for every_row in &board.board_vectors {
            println!("{:?}", every_row);
        }
        //println!("{:?}", board);
        board = board.new_board();
    }
}
#[derive(Debug)]
struct Board {
    board_vectors: Vec<Vec<i32>>,
    size: i32,
}

impl Board {
    
    fn starting_board(size: i32) -> Self {
        let mut start_matrix = vec![vec![0; size as usize];size as usize];
        let initial_live_cells = [(0,1), (1,2), (2,0), (2,1), (2,2)];

        for (x, y) in initial_live_cells {
            start_matrix[x][y] = 1;
        }
        
        Board {board_vectors: start_matrix, size: size}
    }

    fn counting_alive_neighbors(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        let size = 20;
        for every_x in [-1,0,1] {
            for every_y in [-1,0,1]{
                if every_x == 0 && every_y == 0 {
                    continue;
                }
                let new_x = (x - every_x + size) % size;
                let new_y = (y - every_y + size) % size;
                count += self.board_vectors[new_x as usize][new_y as usize];
            }
        }
        count
    }
    
    fn new_board(&self) -> Board {
        let mut new_matrix_board = vec![vec![0; self.size.try_into().unwrap()]; self.size.try_into().unwrap()];
        for x in 0..20 {
            for y in 0..20 {
                let alive_neighbors = self.counting_alive_neighbors(x, y);

                if alive_neighbors == 2 {
                    new_matrix_board[x as usize][y as usize] = self.board_vectors[x as usize][y as usize];
                } else if alive_neighbors == 3{
                    new_matrix_board[x as usize][y as usize] = 1;
                } else{
                    new_matrix_board[x as usize][y as usize] = 0;
                }
            }
        }
        Board{board_vectors: new_matrix_board, size: self.size}
    }
}
