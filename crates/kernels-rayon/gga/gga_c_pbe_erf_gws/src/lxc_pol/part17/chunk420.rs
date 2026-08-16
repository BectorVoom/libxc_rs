//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 420/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk420(t1235: f64, t103: f64, t1251: f64, t1: f64, t120: f64, t485: f64, t119: f64, t155: f64, t481: f64, t1243: f64, t486: f64, t102: f64, t128: f64, t1504: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1508 = param_hyb_omega_0 * t1235;
    let t1509 = t1508 * t103;
    let t1511 = 0.32478055555555555555e0_f64 * t1509 * t1251;
    let t1513 = t485 * t120 * t1;
    let t1515 = t119 * t155 * t481;
    let t1516 = t1513 * t1515;
    let t1517 = 0.97434166666666666666e0_f64 * t1516;
    let t1519 = 0.64956111111111111111e0_f64 * t486 * t1243;
    let t1522 = 0.584605e1_f64 * t102 * t128 * t1504;
    (t1508, t1509, t1511, t1513, t1515, t1516, t1517, t1519, t1522)
}
