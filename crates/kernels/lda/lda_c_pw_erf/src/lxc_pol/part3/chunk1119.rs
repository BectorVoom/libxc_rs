//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1119/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1119<F: Float>(t14040: F, t14042: F, t14045: F, t14047: F, t14050: F, t14053: F, t14054: F, t14055: F, t14056: F, t14058: F, t14066: F, t14070: F, t14072: F, t256: F, t3932: F, t850: F) -> (F, F) {
    let t15121 = -t14040 - t14042 - t14045 - t14047 - t14050 + t14053 + t14054 - t14055 - t14056 - t14058 + t14066 + t14070 - t14072;
    let t15123 = t850 * t3932 * t256;
    (t15121, t15123)
}
