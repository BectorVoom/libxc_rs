//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1239/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1239(t13140: f64, t18281: f64, t18284: f64, t20493: f64, t20495: f64, t20497: f64, t20499: f64, t20501: f64, t20503: f64, t20504: f64, t20505: f64, t20506: f64) -> f64 {
    let t22000 = t18281 + 0.36466666666666664_f64 * t18284 - t20493 - t20495 - t20497 - t20499 - t20501 - t20503 + t20504 - t20505 - t13140 + t20506;
    t22000
}
