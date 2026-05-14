//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 738/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk738<F: Float>(t4645: F, t5260: F, t439: F, t1901: F, t4655: F, t2010: F, t1074: F, t1906: F, t1385: F, t1438: F, t822: F, t1069: F, t2960: F, t1083: F, t1923: F, t1380: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5261 = t5260 * t4645;
    let t5263 = 8.0 / 81.0 * t439 * t5261;
    let t5264 = t1901 * t4655;
    let t5266 = 4.0 / 27.0 * t2010 * t5264;
    let t5267 = t1906 * t1074;
    let t5268 = t1385 * t5267;
    let t5270 = t439 * t5268 / 45.0;
    let t5271 = t822 * t1438;
    let t5272 = t5271 * t1069;
    let t5273 = t2960 * t5272;
    let t5275 = t439 * t5273 / 27.0;
    let t5276 = t1923 * t1083;
    let t5277 = t1380 * t5276;
    (t5261, t5263, t5264, t5266, t5267, t5268, t5270, t5272, t5273, t5275, t5276, t5277)
}
