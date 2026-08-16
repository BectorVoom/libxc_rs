//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1318/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1318(t2271: f64, t343: f64, t2277: f64, t11411: f64, t11419: f64, t11422: f64, t11437: f64, t11445: f64, t11448: f64, t1820: f64, t1823: f64, t1826: f64, t1829: f64, t2268: f64, t2274: f64, t2954: f64, t2961: f64, t2967: f64, t2973: f64, t348: f64, t352: f64, t39: f64, t462: f64, t5812: f64, t5823: f64, t659: f64, t661: f64, t753: f64, t754: f64, t92: f64, t93: f64, t9456: f64, t9481: f64) -> f64 {
    let t15180 = 32.0_f64 * t2271 * t343;
    let t15193 = 32.0_f64 * t2277 * t343;
    let t15198 = 20.0_f64 / 9.0_f64 * t2274 * t2973 + 16.0_f64 * t661 * t39 + 20.0_f64 / 9.0_f64 * t2268 * t2961 - 16.0_f64 * t659 * t39 - 40.0_f64 / 81.0_f64 * t1820 * t2954 + 40.0_f64 / 9.0_f64 * t753 * t9481 + 80.0_f64 / 9.0_f64 * t1823 * t11411 + 40.0_f64 / 3.0_f64 * t92 * t462 * t348 + 40.0_f64 / 3.0_f64 * t5812 * t11422 + t15180 - 40.0_f64 / 81.0_f64 * t1826 * t2967 + 40.0_f64 / 9.0_f64 * t754 * t9456 - 80.0_f64 / 9.0_f64 * t1829 * t11437 - 40.0_f64 / 3.0_f64 * t93 * t462 * t352 - 40.0_f64 / 3.0_f64 * t5823 * t11448 - t15193 - 40.0_f64 * t5812 * t11419 + 40.0_f64 * t5823 * t11445;
    t15198
}
