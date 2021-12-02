// create a sudoku game with gui

// import required modules for gui
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

// import required modules for sudoku game
use sudoku::Sudoku;

// create a sudoku game with gui
pub struct SudokuGuiGame {
    sudoku: Sudoku,
    current_number: u8,
    current_index: (usize, usize),
    selected_index: Option<(usize, usize)>,
    win: bool,
}

impl SudokuGuiGame {
    pub fn new() -> SudokuGuiGame {
        let sudoku = Sudoku::new();
        let current_number = 0;
        let current_index = (0, 0);
        let selected_index = None;
        let win = false;
        SudokuGuiGame {
            sudoku,
            current_number,
            current_index,
            selected_index,
            win,
        }
    }

    pub fn get_sudoku(&self) -> &Sudoku {
        &self.sudoku
    }

    pub fn get_sudoku_mut(&mut self) -> &mut Sudoku {
        &mut self.sudoku
    }

    pub fn get_current_number(&self) -> u8 {
        self.current_number
    }

    pub fn get_current_index(&self) -> (usize, usize) {
        self.current_index
    }

    pub fn get_selected_index(&self) -> Option<(usize, usize)> {
        self.selected_index
    }

    pub fn get_win(&self) -> bool {
        self.win
    }

    pub fn set_current_number(&mut self, current_number: u8) {
        self.current_number = current_number;
    }

    pub fn set_current_index(&mut self, current_index: (usize, usize)) {
        self.current_index = current_index;
    }

    pub fn set_selected_index(&mut self, selected_index: Option<(usize, usize)>) {
        self.selected_index = selected_index;
    }

    pub fn set_win(&mut self, win: bool) {
        self.win = win;
    }

    pub fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        area: Rect,
        terminal: &mut Terminal<B>,
    ) {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(area);

            let top_left = chunks[0];
            let top_right = chunks[1];
            let bottom = chunks[2];

            SudokuGuiGame::draw_top_left(&self.sudoku, &mut f, top_left);
            SudokuGuiGame::draw_top_right(&self.sudoku, &mut f, top_right);
            SudokuGuiGame::draw_bottom(&self.sudoku, &mut f, bottom);
        });
    }
    
    fn draw_top_left<B: Backend>(
        sudoku: &Sudoku,
        f: &mut Frame<B>,
        area: Rect,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
            .split(area);

        let top_left_1 = chunks[0];
        let top_left_2 = chunks[1];
        let top_left_3 = chunks[2];

        SudokuGuiGame::draw_top_left_1(&sudoku, f, top_left_1);
        SudokuGuiGame::draw_top_left_2(&sudoku, f, top_left_2);
        SudokuGuiGame::draw_top_left_3(&sudoku, f, top_left_3);
    }

    fn draw_top_left_1<B: Backend>(
        sudoku: &Sudoku,
        f: &mut Frame<B>,
        area: Rect,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
            .split(area);

        let top_left_1_1 = chunks[0];
        let top_left_1_2 = chunks[1];
        let top_left_1_3 = chunks[2];

        SudokuGuiGame::draw_top_left_1_1(&sudoku, f, top_left_1_1);
        SudokuGuiGame::draw_top_left_1_2(&sudoku, f, top_left_1_2);
        SudokuGuiGame::draw_top_left_1_3(&sudoku, f, top_left_1_3);
    }

    fn draw_top_left_1_1<B: Backend>(
        sudoku: &Sudoku,
        f: &mut Frame<B>,
        area: Rect,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
            .split(area);

        let top_left_1_1_1 = chunks[0];
        let top_left_1_1_2 = chunks[1];
        let top_left_1_1_3 = chunks[2];

        SudokuGuiGame::draw_top_left_1_1_1(&sudoku, f, top_left_1_1_1);
        SudokuGuiGame::draw_top_left_1_1_2(&sudoku, f, top_left_1_1_2);
        SudokuGuiGame::draw_top_left_1_1_3(&sudoku, f, top_left_1_1_3);
    }

    fn draw_top_left_1_1_1<B: Backend>(
        sudoku: &Sudoku,
        f: &mut Frame<B>,
        area: Rect,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
            .split(area);

        let top_left_1_1_1_1 = chunks[0];
        let top_left_1_1_1_2 = chunks[1];
        let top_left_1_1_1_3 = chunks[2];

        SudokuGuiGame::draw_top_left_1_1_1_1(&sudoku, f, top_left_1_1_1_1);
        SudokuGuiGame::draw_top_left_1_1_1_2(&sudoku, f, top_left_1_1_1_2);
        SudokuGuiGame::draw_top_left_1_1_1_3(&sudoku, f, top_left_1_1_1_3);
    }
}