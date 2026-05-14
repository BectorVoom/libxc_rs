//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 979/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk979<F: Float>(t13291: F, t13277: F, t13279: F, t13283: F, t13284: F, t13285: F, t13286: F, t13287: F, t13288: F, t13289: F, t13290: F, t3155: F, t831: F, t1395: F, t1531: F, t5077: F, t5086: F) -> (F, F, F, F) {
    let t13292 = t13291 / 45.0;
    let t13293 = -t13277 - t13279 - t13283 - t13284 - t13285 + t13286 + t13287 - t13288 + t13289 + t13290 - t13292;
    let t13294 = t831 * t3155;
    let t13295 = t13294 / 45.0;
    let t13296 = t1395 * t1531;
    let t13299 = 4.0 / 15.0 * t5077 * t13296 * t5086;
    (t13292, t13293, t13295, t13299)
}
