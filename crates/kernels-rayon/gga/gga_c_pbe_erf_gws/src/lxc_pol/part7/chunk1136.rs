//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1136/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1136(t6336: f64, t6605: f64, t19803: f64, t346: f64, t2382: f64, t6570: f64, t19626: f64, t822: f64, t6299: f64, t6402: f64, t20296: f64, t2157: f64) -> (f64, f64, f64, f64, f64) {
    let t20376 = t6336 * t6605;
    let t20377 = 7.0_f64 / 24.0_f64 * t20376;
    let t20378 = t19803 * t346;
    let t20381 = 11.0_f64 / 96.0_f64 * t2382 * t20378 * t6570;
    let t20382 = t19626 * t346;
    let t20385 = t822 * t20382 * t6570 / 16.0_f64;
    let t20386 = t6402 * t6299;
    let t20388 = t20296 * t2157;
    (t20377, t20381, t20385, t20386, t20388)
}
