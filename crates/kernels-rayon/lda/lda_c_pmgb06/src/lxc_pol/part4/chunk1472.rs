//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1472/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1472(t123: f64, t4429: f64, t868: f64, t199: f64, t315: f64, t6716: f64, t566: f64, t7113: f64, t4454: f64, t1808: f64, t2281: f64, t18057: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18988 = t123 * t4429 * t868;
    let t18995 = t123 * t315 * t6716 * t199;
    let t18998 = t123 * t7113 * t566;
    let t19004 = t123 * t4454 * t868;
    let t19007 = t123 * t2281 * t1808;
    let t19017 = t123 * t18057 * t199;
    (t18988, t18995, t18998, t19004, t19007, t19017)
}
