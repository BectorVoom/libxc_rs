//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 791/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk791<F: Float>(t2960: F, t5272: F, t439: F, t1083: F, t1923: F, t1380: F, t493: F, t1464: F, t851: F, t1080: F, t2991: F, t1420: F, t1894: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5273 = t2960 * t5272;
    let t5275 = t439 * t5273 / F::new(27.0);
    let t5276 = t1923 * t1083;
    let t5277 = t1380 * t5276;
    let t5279 = t493 * t5277 / F::new(45.0);
    let t5280 = t851 * t1464;
    let t5281 = t5280 * t1080;
    let t5282 = t2991 * t5281;
    let t5284 = t493 * t5282 / F::new(27.0);
    let t5286 = F::new(2.0) / F::new(45.0) * t1420 * t1894;
    (t5273, t5275, t5276, t5277, t5279, t5281, t5282, t5284, t5286)
}
