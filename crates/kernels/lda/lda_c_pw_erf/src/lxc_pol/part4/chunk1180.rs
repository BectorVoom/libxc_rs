//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1180/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1180<F: Float>(t4729: F, t795: F, t5184: F, t17401: F, t17402: F, t17403: F, t17406: F, t17409: F, t17412: F, t17414: F, t17415: F, t17416: F, t17418: F, t17422: F, t17424: F, t17427: F, t17431: F, t17432: F) -> (F, F, F) {
    let t17434 = t795 * t4729;
    let t17435 = 8.0 / 135.0 * t17434;
    let t17436 = t795 * t5184;
    let t17437 = 16.0 / 45.0 * t17436;
    let t17438 = t17401 + t17402 + t17403 - t17406 + t17409 - t17412 - t17414 + t17415 + t17416 + t17418 - t17422 - t17424 - t17427 + t17431 + 8.0 / 3.0 * t17432 + t17435 - t17437;
    (t17435, t17437, t17438)
}
