//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 324/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk324(t1: f64, t1382: f64, t297: f64, t313: f64, t1235: f64, t897: f64, t894: f64, t1379: f64, t860: f64, t862: f64, t874: f64, t891: f64, t893: f64) -> (f64, f64, f64, f64, f64) {
    let t1383 = t1382 * t1;
    let t1384 = t1383 * t297;
    let t1385 = t313 * t1384;
    let t1388 = t897 * t1235;
    let t1389 = t894 * t1388;
    let t1392 = t860 + t862 * t1379 / 288.0_f64 + 0.35500316489081544176e-1_f64 * t874 * t1385 + t891 + 0.18110753103726578864e-2_f64 * t893 * t1389;
    (t1383, t1384, t1388, t1389, t1392)
}
