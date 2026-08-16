//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1102/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1102(t1915: f64, t19332: f64, t493: f64, t1919: f64, t19385: f64, t1897: f64, t19770: f64, t2010: f64, t1380: f64, t529: f64, t7605: f64, t12869: f64, t12871: f64, t20235: f64, t20238: f64, t20241: f64, t20243: f64, t20247: f64) -> (f64, f64, f64, f64, f64) {
    let t20250 = 8.0_f64 / 15.0_f64 * t493 * t1915 * t19332;
    let t20253 = 4.0_f64 / 3.0_f64 * t493 * t1919 * t19385;
    let t20256 = 4.0_f64 / 5.0_f64 * t2010 * t1897 * t19770;
    let t20260 = 2.0_f64 / 15.0_f64 * t493 * t1380 * t7605 * t529;
    let t20261 = -t20235 - t20238 + t20241 - t20243 - t20247 - t20250 + t20253 + t20256 - t20260 + t12869 + t12871;
    (t20250, t20253, t20256, t20260, t20261)
}
