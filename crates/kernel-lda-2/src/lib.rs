#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! LDA kernel translations: kernel-lda-2 (6 functionals).

pub mod lda_c_pk09;
pub mod lda_c_pmgb06;
pub mod lda_c_pw_erf;
pub mod lda_c_vwn_1;
pub mod lda_c_wigner;
pub mod lda_xc_ksdt;
