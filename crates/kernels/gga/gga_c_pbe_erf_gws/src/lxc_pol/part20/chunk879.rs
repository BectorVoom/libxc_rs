//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 879/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk879<F: Float>(t810: F, t824: F, t2118: F, t814: F, t3224: F, t6402: F, t3287: F, t6203: F, t3232: F, t6627: F, t3237: F, t2289: F, t3283: F) -> (F, F, F, F, F, F, F) {
    let t9504 = t824 * t810;
    let t9520 = t2118 * t814;
    let t9539 = F::new(7.0) / F::new(576.0) * t6402 * t3224;
    let t9549 = F::new(7.0) / F::new(288.0) * t6203 * t3287;
    let t9565 = F::new(7.0) / F::new(288.0) * t6627 * t3232;
    let t9579 = F::new(7.0) / F::new(1152.0) * t6627 * t3237;
    let t9592 = F::new(7.0) / F::new(1152.0) * t2289 * t3283;
    (t9504, t9520, t9539, t9549, t9565, t9579, t9592)
}
