//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1089/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1089<F: Float>(t16255: F, t16259: F, t16262: F, t16263: F, t16264: F, t16266: F, t16271: F, t16275: F, t16276: F, t16277: F, t16278: F, t16282: F, t16285: F, t16287: F, t16289: F, t486: F, t6596: F) -> (F, F) {
    let t16290 = t16255 + t16259 + t16262 + t16263 + t16264 - t16266 - t16271 - t16275 + t16276 - t16277 - t16278 - t16282 - t16285 - t16287 - t16289;
    let t16293 = t486 * t6596 / 15.0;
    (t16290, t16293)
}
