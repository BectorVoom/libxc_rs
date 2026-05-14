//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1273/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1273<F: Float>(t15460: F, t15462: F, t15466: F, t15467: F, t15468: F, t15472: F, t15473: F, t15474: F, t15475: F, t15476: F, t8373: F, t8382: F, t8386: F, t8389: F, t8393: F, t8397: F, t8400: F) -> (F,) {
    let t18963 = -t15460 - t15462 - t8373 - t8382 + t8386 - t15466 - t15467 - t15468 - t8389 - t8393 + t8397 - t8400 - t15472 + t15473 - t15474 + t15475 + t15476;
    (t18963,)
}
