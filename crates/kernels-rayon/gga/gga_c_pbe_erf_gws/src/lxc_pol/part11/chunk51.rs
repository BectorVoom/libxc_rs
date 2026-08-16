//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 51/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk51(t108: f64, t116: f64, t3: f64, t5: f64, t99: f64, t1: f64) -> (f64, f64) {
    let pi = (M_PI as f64);
    let t117 = (0.344851e1_f64 - pi * t5 * t108 * t3 / t99 / 12.0_f64) * t116;
    let t118 = t117 * t1;
    (t117, t118)
}
