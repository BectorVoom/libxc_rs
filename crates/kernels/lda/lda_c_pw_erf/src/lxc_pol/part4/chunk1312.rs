//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1312/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1312<F: Float>(t17395: F, t17397: F, t17399: F, t17401: F, t17402: F, t17403: F, t17406: F, t17409: F, t17412: F, t17414: F, t17415: F, t17416: F, t17418: F, t17422: F, t17424: F, t17427: F, t17431: F) -> (F,) {
    let t19248 = -t17395 - t17397 - t17399 + t17401 + t17402 + t17403 - t17406 + t17409 - t17412 - t17414 + t17415 + t17416 + t17418 - t17422 - t17424 - t17427 + t17431;
    (t19248,)
}
