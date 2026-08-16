//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 445/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk445(t1098: f64, t19: f64, t796: f64, t801: f64, t1402: f64, t950: f64, t1412: f64, t954: f64, t1523: f64, t1528: f64, t1143: f64, t376: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2454 = t1098 * t796 * t19;
    let t2455 = t2454 * t801;
    let t2457 = t1402 * t950;
    let t2465 = t1412 * t954;
    let t2477 = t1523 * t950;
    let t2485 = t1528 * t954;
    let t2501 = t1143 * t376;
    (t2454, t2455, t2457, t2465, t2477, t2485, t2501)
}
