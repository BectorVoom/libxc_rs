//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1047/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1047(t18969: f64, t408: f64, t4259: f64, t88: f64, t18699: f64, t85: f64, t414: f64, t4743: f64, t428: f64, t4358: f64, t1336: f64, t1423: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18970 = 72.0_f64 * t18969;
    let t18972 = t408 * t4259 * t88;
    let t18973 = 1920.0_f64 * t18972;
    let t18975 = 0.19751789702565206229e-1_f64 * t18699 * t85;
    let t18977 = 16.0_f64 * t414 * t4743;
    let t18978 = t4358 * t428;
    let t18979 = 96.0_f64 * t18978;
    let t18980 = t1336 * t1423;
    (t18970, t18973, t18975, t18977, t18979, t18980)
}
