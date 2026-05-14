//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1093/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1093<F: Float>(t1318: F, t1319: F, t15931: F, t352: F, t4753: F, t6277: F, t3416: F, t565: F, t6845: F, t11954: F, t2146: F, t5371: F, t1454: F, t6198: F, t1462: F, t1341: F, t6988: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15935 = 16.0 / 45.0 * t1318 * t1319 * t15931 * t352;
    let t15937 = 16.0 / 45.0 * t4753 * t6277;
    let t15939 = 16.0 / 45.0 * t3416 * t6277;
    let t15941 = 4.0 / 15.0 * t565 * t6845;
    let t15942 = 16.0 / 135.0 * t11954;
    let t15943 = t2146 * t5371;
    let t15944 = 32.0 / 45.0 * t15943;
    let t15946 = 4.0 / 45.0 * t6198 * t1454;
    let t15948 = 4.0 / 27.0 * t6198 * t1462;
    let t15950 = 16.0 / 45.0 * t6988 * t1341;
    (t15935, t15937, t15939, t15941, t15942, t15944, t15946, t15948, t15950)
}
