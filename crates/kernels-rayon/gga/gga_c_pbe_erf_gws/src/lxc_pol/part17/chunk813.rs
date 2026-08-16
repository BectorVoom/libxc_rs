//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 813/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk813(t2268: f64, t6501: f64, t2331: f64, t899: f64, t900: f64, t935: f64, t2323: f64, t2327: f64, t2206: f64, t2212: f64, t2285: f64, t2289: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6502 = t6501 * t2268;
    let t6505 = t899 * t900 * t2331;
    let t6506 = t6505 * t935;
    let t6508 = t2323 * t2327;
    let t6510 = t2206 * t2212;
    let t6517 = t2289 * t2285;
    (t6502, t6505, t6506, t6508, t6510, t6517)
}
