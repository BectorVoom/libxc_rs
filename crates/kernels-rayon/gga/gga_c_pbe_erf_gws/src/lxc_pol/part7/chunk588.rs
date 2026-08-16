//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 588/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk588(t2362: f64, t2384: f64, t2388: f64, t2397: f64, t4385: f64, t4390: f64, t4397: f64, t4402: f64, t4405: f64, t4410: f64, t4415: f64, t4419: f64, t4425: f64, t4427: f64, t4430: f64, t4433: f64, t4438: f64, t4443: f64, t4446: f64, t4454: f64, t827: f64, t833: f64, t844: f64) -> f64 {
    let t4456 = t4385 * t4390 / 16.0_f64 - t4397 * t2362 / 32.0_f64 - t827 * t4402 / 32.0_f64 - t4405 * t2362 / 48.0_f64 - t4410 * t2362 / 48.0_f64 - 7.0_f64 / 24.0_f64 * t4415 + t827 * t4419 / 32.0_f64 + 35.0_f64 / 144.0_f64 * t4425 + t4427 * t833 / 48.0_f64 - 35.0_f64 / 144.0_f64 * t4430 - t844 * t4433 / 16.0_f64 - t844 * t4438 / 48.0_f64 - 35.0_f64 / 72.0_f64 * t4443 - t844 * t4446 / 16.0_f64 + t2384 * t2397 / 32.0_f64 + t2388 * t2397 / 32.0_f64 - 7.0_f64 / 96.0_f64 * t4454;
    t4456
}
