#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! LDA kernel translations: kernel-lda-1 (9 functionals).

pub mod hyb_lda_xc_bn05;
pub mod lda_c_ml1;
pub mod lda_c_vwn;
pub mod lda_c_vwn_2;
pub mod lda_c_vwn_3;
pub mod lda_c_vwn_4;
pub mod lda_c_vwn_rpa;
pub mod lda_c_w20;
pub mod lda_x_yukawa;
