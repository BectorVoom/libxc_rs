//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 696/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk696(t168: f64, t5589: f64, t286: f64, t159: f64, t285: f64, t4562: f64, t4353: f64, t4356: f64, t4361: f64, t4368: f64, t4371: f64, t4374: f64) -> (f64, f64, f64, f64) {
    let t5631 = t168 * t5589;
    let t5633 = 0.19513566535229733338e0_f64 * t5631 * t286;
    let t5636 = t4562 * t159 * t285;
    let t5645 = 4.0_f64 / 27.0_f64 * t4353 - t4356 / 3.0_f64 + t4361 / 3.0_f64 + 4.0_f64 / 27.0_f64 * t4368 - t4371 / 3.0_f64 + t4374 / 3.0_f64;
    (t5631, t5633, t5636, t5645)
}
