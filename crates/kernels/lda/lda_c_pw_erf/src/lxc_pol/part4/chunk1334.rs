//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1334/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1334<F: Float>(t18363: F, t18365: F, t18367: F, t18369: F, t18373: F, t18378: F, t18383: F, t18388: F, t18391: F, t18394: F, t18396: F, t18398: F, t18399: F, t18401: F, t18403: F, t18405: F, t18408: F) -> (F,) {
    let t19302 = -t18363 + t18365 - t18367 - t18369 - t18373 + t18378 + t18383 + t18388 - t18391 + t18394 + t18396 - t18398 + t18399 + t18401 + t18403 - t18405 - t18408;
    (t19302,)
}
