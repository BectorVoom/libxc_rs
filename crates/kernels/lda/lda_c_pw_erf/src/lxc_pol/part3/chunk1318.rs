//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1318/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1318<F: Float>(t2271: F, t343: F, t2277: F, t11411: F, t11419: F, t11422: F, t11437: F, t11445: F, t11448: F, t1820: F, t1823: F, t1826: F, t1829: F, t2268: F, t2274: F, t2954: F, t2961: F, t2967: F, t2973: F, t348: F, t352: F, t39: F, t462: F, t5812: F, t5823: F, t659: F, t661: F, t753: F, t754: F, t92: F, t93: F, t9456: F, t9481: F) -> F {
    let t15180 = F::cast_from(32.0_f64) * t2271 * t343;
    let t15193 = F::cast_from(32.0_f64) * t2277 * t343;
    let t15198 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t2274 * t2973 + F::cast_from(16.0_f64) * t661 * t39 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t2268 * t2961 - F::cast_from(16.0_f64) * t659 * t39 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1820 * t2954 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t753 * t9481 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t1823 * t11411 + F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t92 * t462 * t348 + F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t5812 * t11422 + t15180 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1826 * t2967 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t754 * t9456 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t1829 * t11437 - F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t93 * t462 * t352 - F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t5823 * t11448 - t15193 - F::cast_from(40.0_f64) * t5812 * t11419 + F::cast_from(40.0_f64) * t5823 * t11445;
    t15198
}
