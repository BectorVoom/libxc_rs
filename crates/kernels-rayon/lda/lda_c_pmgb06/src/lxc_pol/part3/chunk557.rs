//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 557/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk557(t2912: f64, t2918: f64, t1476: f64, t36: f64, t1464: f64, t337: f64, t1083: f64) -> (f64, f64, f64, f64, f64) {
    let t2919 = t2918 * t2912;
    let t2920 = t1476 * t2919;
    let t2921 = t36 * t2920;
    let t2923 = t1464 * t337;
    let t2924 = t2923 * t1083;
    (t2919, t2920, t2921, t2923, t2924)
}
