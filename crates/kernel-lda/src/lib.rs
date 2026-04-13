#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

// LDA kernel implementations.
// Canonical reference (hand-translated):
pub mod lda_x;
pub mod launch_lda_x;

// Auto-translated from maple2c C sources:
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
// pub mod lda_x_1d_exponential; // Uses xc_integrate (numerical quadrature) -- needs CPU-only fallback
// pub mod lda_x_1d_soft;        // Uses xc_integrate + xc_bessel_K -- needs CPU-only fallback
pub mod lda_x_2d;
pub mod lda_x_erf;
pub mod lda_x_rel;
pub mod lda_x_sloc;
pub mod lda_x_yukawa;
pub mod lda_xc_1d_ehwlrg;
pub mod lda_xc_teter93;
pub mod lda_xc_tih;
pub mod lda_xc_zlp;

// Large kernels -- deferred (OOM during compilation even with 16GB RAM):
// pub mod lda_c_pmgb06;  // lxc_pol: 9.8K lines
// pub mod lda_c_pw_erf;  // lxc_pol: 11K lines

// Extremely large kernels -- deferred (individual functions exceed compiler memory limits):
// pub mod lda_c_pk09;   // kxc_pol: 17K lines
// pub mod lda_xc_ksdt;  // lxc_pol: 14K lines
