//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1059/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1059<F: Float>(t8206: F, t8208: F, t8210: F, t8212: F, t8214: F, t8216: F, t11325: F, t15408: F, t15410: F, t15413: F, t15415: F, t15416: F, t15417: F, t15418: F, t15419: F, t15420: F, t15421: F, t15423: F, t15424: F, t8202: F) -> (F, F, F, F, F, F, F) {
    let t15425 = 8.0 * t8206;
    let t15426 = 20.0 * t8208;
    let t15427 = 32.0 * t8210;
    let t15428 = 240.0 * t8212;
    let t15429 = 8.0 * t8214;
    let t15430 = 24.0 * t8216;
    let t15431 = -t15408 - 3.651229230228148 * t11325 + t15410 - 0.8215265768013333 * t15413 - t15415 + t15416 + t15417 - t15418 - t15419 - t15420 + 0.6846054806677778 * t15421 + t15423 - t8202 - t15424 - t15425 + t15426 + t15427 - t15428 - t15429 - t15430;
    (t15425, t15426, t15427, t15428, t15429, t15430, t15431)
}
