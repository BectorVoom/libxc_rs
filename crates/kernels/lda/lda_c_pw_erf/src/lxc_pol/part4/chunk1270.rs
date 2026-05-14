//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1270/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1270<F: Float>(t15408: F, t15410: F, t15415: F, t15416: F, t15417: F, t15418: F, t15419: F, t15420: F, t15423: F, t15424: F, t15425: F, t15426: F, t15427: F, t15428: F, t15429: F, t15430: F, t15433: F, t8202: F) -> (F,) {
    let t18957 = -t15408 + t15410 - t15415 + t15416 + t15417 - t15418 - t15419 - t15420 + t15423 - t8202 - t15424 - t15425 + t15426 + t15427 - t15428 - t15429 - t15430 + t15433;
    (t18957,)
}
