//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 93/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk93<F: Float>(t190: F, t192: F, t207: F, t212: F) -> (F, F) {
    let t218 = F::cast_from(1.0_f64) + F::cast_from(0.107975e0_f64) * t207 + F::cast_from(0.1e-1_f64) * t190 * t192 * t212;
    let t219 = F::cast_from(1.0_f64) / t218;
    (t218, t219)
}
