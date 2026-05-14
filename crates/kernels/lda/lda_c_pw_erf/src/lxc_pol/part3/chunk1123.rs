//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1123/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1123<F: Float>(t2271: F, t343: F, t2277: F, t11411: F, t11419: F, t11422: F, t11437: F, t11445: F, t11448: F, t1820: F, t1823: F, t1826: F, t1829: F, t2268: F, t2274: F, t2954: F, t2961: F, t2967: F, t2973: F, t348: F, t352: F, t39: F, t462: F, t5812: F, t5823: F, t659: F, t661: F, t753: F, t754: F, t92: F, t93: F, t9456: F, t9481: F) -> (F,) {
    let t15180 = 32.0 * t2271 * t343;
    let t15193 = 32.0 * t2277 * t343;
    let t15198 = 20.0 / 9.0 * t2274 * t2973 + 16.0 * t661 * t39 + 20.0 / 9.0 * t2268 * t2961 - 16.0 * t659 * t39 - 40.0 / 81.0 * t1820 * t2954 + 40.0 / 9.0 * t753 * t9481 + 80.0 / 9.0 * t1823 * t11411 + 40.0 / 3.0 * t92 * t462 * t348 + 40.0 / 3.0 * t5812 * t11422 + t15180 - 40.0 / 81.0 * t1826 * t2967 + 40.0 / 9.0 * t754 * t9456 - 80.0 / 9.0 * t1829 * t11437 - 40.0 / 3.0 * t93 * t462 * t352 - 40.0 / 3.0 * t5823 * t11448 - t15193 - 40.0 * t5812 * t11419 + 40.0 * t5823 * t11445;
    (t15198,)
}
