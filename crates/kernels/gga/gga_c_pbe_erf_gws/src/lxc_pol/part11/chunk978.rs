//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 978/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk978<F: Float>(t40771: F, t40773: F, t40783: F, t31443: F, t3354: F) -> (F, F, F, F, F) {
    let t47729 = 32.0 / 45.0 * t40771;
    let t47730 = 128.0 / 45.0 * t40773;
    let t47731 = 32.0 / 15.0 * t40783;
    let t47732 = 16.0 / 45.0 * t31443;
    let t47733 = t3354 * t3354;
    (t47729, t47730, t47731, t47732, t47733)
}
