//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 553/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk553<F: Float>(t2954: F, t2961: F, t2967: F, t2973: F, t3234: F, t3237: F, t3243: F, t3246: F, t406: F, t408: F, t945: F, t954: F) -> (F,) {
    let t3251 = 4.0 / 27.0 * t3234 * t2954 - t3237 * t945 / 3.0 + t406 * t2961 / 3.0 + 4.0 / 27.0 * t3243 * t2967 - t3246 * t954 / 3.0 + t408 * t2973 / 3.0;
    (t3251,)
}
