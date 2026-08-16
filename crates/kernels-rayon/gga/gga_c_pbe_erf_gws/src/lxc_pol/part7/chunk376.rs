//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 376/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk376(t119: f64, t155: f64, t481: f64, t1513: f64, t1243: f64, t486: f64, t102: f64, t128: f64, t1504: f64, t48: f64, t1403: f64, t1407: f64, t476: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1515 = t119 * t155 * t481;
    let t1516 = t1513 * t1515;
    let t1517 = 0.97434166666666666666e0_f64 * t1516;
    let t1519 = 0.64956111111111111111e0_f64 * t486 * t1243;
    let t1522 = 0.584605e1_f64 * t102 * t128 * t1504;
    let t1523 = 1.0_f64 / t48;
    let t1524 = t1523 * t1403;
    let t1526 = t476 * t1407;
    (t1515, t1517, t1519, t1522, t1523, t1524, t1526)
}
