//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1152/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1152(t161: f64, t16595: f64, t166: f64, t851: f64, t479: f64, t7465: f64, t2108: f64, t2592: f64, t486: f64, t7443: f64, t13182: f64, t2469: f64, t493: f64) -> (f64, f64, f64, f64, f64) {
    let t20843 = t161 * t166 * t16595 * t851 / 10.0_f64;
    let t20845 = t7465 * t479 / 30.0_f64;
    let t20847 = t2592 * t2108 / 10.0_f64;
    let t20849 = t486 * t7443 / 10.0_f64;
    let t20852 = t493 * t13182 * t2469 / 9.0_f64;
    (t20843, t20845, t20847, t20849, t20852)
}
