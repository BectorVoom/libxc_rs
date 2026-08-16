//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 969/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk969(t8370: f64, t8374: f64, t11441: f64, t8353: f64, t8358: f64, t8376: f64, t8379: f64, t8382: f64, t8386: f64, t8388: f64, t8390: f64, t3631: f64, t783: f64) -> (f64, f64, f64, f64) {
    let t11443 = 3.8973666666666666_f64 * t8370;
    let t11444 = 1.9486833333333333_f64 * t8374;
    let t11451 = 6.85552_f64 * t8353 + t11441 + 14.0_f64 / 9.0_f64 * t8358 + t11443 - t11444 + 11.75232_f64 * t8376 + 2.0_f64 * t8379 + 2.0_f64 * t8382 + 5.87616_f64 * t8386 + 5.87616_f64 * t8388 - 2.93808_f64 * t8390;
    let t11465 = t783 * t3631;
    (t11443, t11444, t11451, t11465)
}
