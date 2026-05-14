//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 860/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk860<F: Float>(t12637: F, t2076: F, t3660: F, t197: F, t4906: F, t10463: F, t1325: F, t2006: F, t1896: F, t603: F, t1440: F, t3675: F, t1390: F, t3787: F, t2026: F, t2022: F, t571: F, t9313: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12638 = 8.0 / 135.0 * t12637;
    let t12684 = t2076 * t3660;
    let t12685 = 8.0 / 45.0 * t12684;
    let t12695 = t4906 * t197;
    let t12708 = t1325 * t10463 * t2006;
    let t12709 = 16.0 / 135.0 * t12708;
    let t12714 = t1896 * t603;
    let t12765 = t1440 * t3675;
    let t12781 = t3787 * t1390;
    let t12809 = t1325 * t10463 * t2026;
    let t12810 = 16.0 / 135.0 * t12809;
    let t12814 = t571 * t9313 * t2022;
    (t12638, t12685, t12695, t12709, t12714, t12765, t12781, t12810, t12814)
}
