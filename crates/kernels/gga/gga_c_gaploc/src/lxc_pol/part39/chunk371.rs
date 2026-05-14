//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 371/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk371<F: Float>(t3226: F, t3217: F, t3218: F, t3223: F, t871: F, t931: F, t295: F, t3113: F, t471: F) -> (F, F, F, F) {
    let t3227 = t3226 / 256.0;
    let t3228 = t3217 - 9.0 / 8192.0 * t3218 + 3.0 / 8192.0 * t3223 - t3227;
    let t3230 = t931 * t871;
    let t3232 = t295 * t3113;
    let t3234 = t3228 * t471 + t3230 / 2.0 + t3217 - t3227 - t3232 / 2.0;
    (t3228, t3230, t3232, t3234)
}
