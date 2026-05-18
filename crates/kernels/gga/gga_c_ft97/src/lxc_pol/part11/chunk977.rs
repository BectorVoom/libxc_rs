//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 977/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk977<F: Float>(t1546: F, t89: F, t9012: F, t37401: F, t9026: F, t363: F, t9348: F, t2075: F, t3139: F, t583: F, t143: F, t37352: F) -> (F, F, F, F, F, F) {
    let t40318 = t89 * t1546 * t9012;
    let t40321 = t89 * t37401 * t9026;
    let t40323 = t9348 * t363;
    let t40327 = t363 * t2075;
    let t40335 = t3139 * t583;
    let t40337 = t37352 * t143;
    (t40318, t40321, t40323, t40327, t40335, t40337)
}
