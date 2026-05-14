//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 739/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk739<F: Float>(t493: F, t5277: F, t1464: F, t851: F, t1080: F, t2991: F, t1420: F, t1894: F, t1893: F, t2948: F, t439: F, t1629: F, t809: F, t1385: F, t1868: F, t477: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5279 = t493 * t5277 / 45.0;
    let t5280 = t851 * t1464;
    let t5281 = t5280 * t1080;
    let t5282 = t2991 * t5281;
    let t5284 = t493 * t5282 / 27.0;
    let t5286 = 2.0 / 45.0 * t1420 * t1894;
    let t5287 = t2948 * t1893;
    let t5289 = 2.0 / 45.0 * t439 * t5287;
    let t5290 = t809 * t1629;
    let t5291 = t1385 * t5290;
    let t5293 = t439 * t5291 / 45.0;
    let t5294 = t1868 * t477;
    (t5279, t5281, t5282, t5284, t5286, t5287, t5289, t5290, t5291, t5293, t5294)
}
