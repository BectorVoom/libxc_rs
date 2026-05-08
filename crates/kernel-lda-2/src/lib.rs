#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! LDA kernel translations: kernel-lda-2 (26 functionals).

pub mod lda_c_1d_csc;
pub mod lda_c_1d_loos;
pub mod lda_c_2d_amgb;
pub mod lda_c_2d_prm;
pub mod lda_c_chachiyo;
pub mod lda_c_chachiyo_mod;
pub mod lda_c_gk72;
pub mod lda_c_gombas;
pub mod lda_c_hl;
pub mod lda_c_lp96;
pub mod lda_c_pw;
pub mod lda_c_pz;
pub mod lda_c_rc04;
pub mod lda_c_rpa;
pub mod lda_c_wigner;
pub mod lda_k_gds08_worker;
pub mod lda_k_tf;
pub mod lda_k_zlp;
pub mod lda_x;
pub mod lda_x_2d;
pub mod lda_x_erf;
pub mod lda_x_rel;
pub mod lda_x_sloc;
pub mod lda_xc_teter93;
pub mod lda_xc_tih;
pub mod lda_xc_zlp;
