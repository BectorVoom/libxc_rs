//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 970/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk970<F: Float>(t1919: F, t19354: F, t493: F, t1972: F, t6770: F, t1380: F, t529: F, t7612: F, t1915: F, t19332: F, t19385: F, t1897: F, t19770: F, t2010: F, t7605: F, t12869: F, t12871: F, t20235: F, t20238: F) -> (F, F, F, F, F, F, F, F) {
    let t20241 = t493 * t1919 * t19354 / 27.0;
    let t20243 = t1972 * t6770 / 9.0;
    let t20247 = t493 * t1380 * t7612 * t529 / 45.0;
    let t20250 = 8.0 / 15.0 * t493 * t1915 * t19332;
    let t20253 = 4.0 / 3.0 * t493 * t1919 * t19385;
    let t20256 = 4.0 / 5.0 * t2010 * t1897 * t19770;
    let t20260 = 2.0 / 15.0 * t493 * t1380 * t7605 * t529;
    let t20261 = -t20235 - t20238 + t20241 - t20243 - t20247 - t20250 + t20253 + t20256 - t20260 + t12869 + t12871;
    (t20241, t20243, t20247, t20250, t20253, t20256, t20260, t20261)
}
