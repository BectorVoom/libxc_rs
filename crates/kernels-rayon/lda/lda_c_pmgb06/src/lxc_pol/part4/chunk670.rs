//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 670/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk670(t3720: f64, t693: f64, t681: f64, t967: f64, t957: f64, t963: f64, t696: f64, t683: f64, t978: f64, t1179: f64, t282: f64, t55: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3721 = t3720 * t693;
    let t3724 = t967 * t681;
    let t3725 = t963 * t957 * t3724;
    let t3727 = 51.94757731704439_f64 * t696 * t3725;
    let t3729 = t978 * t957 * t683;
    let t3731 = 3.5089341735807875_f64 * t696 * t3729;
    let t3734 = t55 * t1179 * t282;
    (t3721, t3724, t3725, t3727, t3729, t3731, t3734)
}
