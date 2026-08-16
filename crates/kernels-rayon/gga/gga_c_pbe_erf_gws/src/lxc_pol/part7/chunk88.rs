//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 88/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk88(t191: f64, t23: f64, t179: f64, t187: f64, t190: f64) -> (f64, f64, f64) {
    let t192 = t23 * t191;
    let t196 = 1.0_f64 + 0.107975e0_f64 * t179 + 0.1e-1_f64 * t190 * t192 * t187;
    let t197 = 1.0_f64 / t196;
    (t192, t196, t197)
}
