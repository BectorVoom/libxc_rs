//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1216/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1216(t11898: f64, t19508: f64, t19509: f64, t19510: f64, t19514: f64, t19518: f64, t19521: f64, t19522: f64, t19524: f64, t19526: f64, t19527: f64, t19528: f64) -> f64 {
    let t21911 = t19508 + t19509 + t19510 + t19514 + t19518 + t19521 + t11898 - t19522 - t19524 - t19526 - t19527 + t19528;
    t21911
}
