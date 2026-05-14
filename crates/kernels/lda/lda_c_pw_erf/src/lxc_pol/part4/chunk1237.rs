//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1237/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1237<F: Float>(t2065: F, t1402: F, t186: F, t211: F, t514: F, t6844: F, t18346: F, t18350: F, t18352: F, t18354: F, t18356: F, t18358: F, t18359: F, t18361: F, t18363: F, t18365: F, t18367: F, t18369: F, t18373: F, t18378: F, t18383: F) -> (F, F, F) {
    let t18384 = t2065 * t2065;
    let t18388 = 8.0 / 15.0 * t211 * t186 * t1402 * t18384;
    let t18390 = t211 * t514 * t6844;
    let t18391 = 8.0 / 45.0 * t18390;
    let t18392 = t18346 + t18350 - t18352 + t18354 + t18356 + t18358 - t18359 - t18361 - t18363 + t18365 - t18367 - t18369 - t18373 + t18378 + t18383 + t18388 - t18391;
    (t18388, t18391, t18392)
}
