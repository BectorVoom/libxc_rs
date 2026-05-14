//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1077/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1077<F: Float>(t518: F, t6208: F, t4959: F, t12615: F, t1325: F, t1278: F, t1440: F, t519: F, t6903: F, t6579: F, t2168: F, t1450: F, t6988: F, t2186: F, t5127: F, t2334: F, t352: F, t593: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15685 = t6208 * t518;
    let t15687 = 16.0 / 15.0 * t15685 * t4959;
    let t15689 = 16.0 / 45.0 * t1325 * t12615;
    let t15693 = 4.0 / 5.0 * t519 * t1440 * t6903 * t1278;
    let t15694 = t6579 * t518;
    let t15696 = 16.0 / 15.0 * t15694 * t2168;
    let t15697 = t6988 * t1450;
    let t15698 = 32.0 / 135.0 * t15697;
    let t15702 = 8.0 / 15.0 * t519 * t1440 * t2186 * t5127;
    let t15704 = t2334 * t593 * t352;
    (t15685, t15687, t15689, t15693, t15694, t15696, t15698, t15702, t15704)
}
