//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 774/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk774<F: Float>(t439: F, t5268: F, t1438: F, t822: F, t1069: F, t2960: F, t1083: F, t1923: F, t1380: F, t493: F, t1464: F, t851: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5270 = t439 * t5268 / F::cast_from(45.0_f64);
    let t5271 = t822 * t1438;
    let t5272 = t5271 * t1069;
    let t5273 = t2960 * t5272;
    let t5275 = t439 * t5273 / F::cast_from(27.0_f64);
    let t5276 = t1923 * t1083;
    let t5277 = t1380 * t5276;
    let t5279 = t493 * t5277 / F::cast_from(45.0_f64);
    let t5280 = t851 * t1464;
    (t5270, t5271, t5272, t5273, t5275, t5276, t5277, t5279, t5280)
}
