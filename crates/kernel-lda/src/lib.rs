#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! LDA kernel implementations (incremental derivative structure).
//! Auto-generated: 41 functionals (37 compiled, 4 deferred).

pub mod hyb_lda_xc_bn05;
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
pub mod lda_c_ml1;
pub mod lda_c_pw;
pub mod lda_c_pz;
pub mod lda_c_rc04;
pub mod lda_c_rpa;
pub mod lda_c_vwn;
pub mod lda_c_vwn_1;
pub mod lda_c_vwn_2;
pub mod lda_c_vwn_3;
pub mod lda_c_vwn_4;
pub mod lda_c_vwn_rpa;
pub mod lda_c_w20;
pub mod lda_c_wigner;
pub mod lda_k_gds08_worker;
pub mod lda_k_tf;
pub mod lda_k_zlp;
pub mod lda_x;
pub mod lda_x_2d;
pub mod lda_x_erf;
pub mod lda_x_rel;
pub mod lda_x_sloc;
pub mod lda_x_yukawa;
pub mod lda_xc_1d_ehwlrg;
pub mod lda_xc_teter93;
pub mod lda_xc_tih;
pub mod lda_xc_zlp;

// Deferred: source generated with incremental structure but individual functions
// exceed CubeCL proc macro stack limits (kxc_pol/lxc_pol > 10K lines).
// pub mod lda_c_pk09;    // kxc_pol: 17.5K lines
// pub mod lda_c_pmgb06;  // lxc_pol: 9.8K lines
// pub mod lda_c_pw_erf;  // lxc_pol: 11K lines
// pub mod lda_xc_ksdt;   // lxc_pol: 14K lines
