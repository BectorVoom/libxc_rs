//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1141/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1141<F: Float>(t108210: F, t108249: F, t108260: F, t108262: F, t108333: F, t108353: F, t108356: F, t108393: F, t108429: F, t108431: F, t108433: F, t109356: F, t109358: F, t109437: F, t109442: F, t109469: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t110103 = 2.0 / 27.0 * t108210;
    let t110125 = t108249 / 27.0;
    let t110128 = t108260 / 9.0;
    let t110129 = 2.0 / 3.0 * t108262;
    let t110151 = 4.0 / 9.0 * t108333;
    let t110159 = 4.0 / 9.0 * t108353;
    let t110160 = 4.0 / 9.0 * t108356;
    let t110169 = 4.0 / 27.0 * t108393;
    let t110182 = 4.0 / 27.0 * t108429;
    let t110183 = 4.0 / 27.0 * t108431;
    let t110184 = 4.0 / 81.0 * t108433;
    let t110201 = t109356 / 12.0;
    let t110202 = t109358 / 9.0;
    let t110235 = 2.0 / 9.0 * t109437;
    let t110238 = t109442 / 9.0;
    let t110245 = 4.0 / 3.0 * t109469;
    (t110103, t110125, t110128, t110129, t110151, t110159, t110160, t110169, t110182, t110183, t110184, t110201, t110202, t110235, t110238, t110245)
}
