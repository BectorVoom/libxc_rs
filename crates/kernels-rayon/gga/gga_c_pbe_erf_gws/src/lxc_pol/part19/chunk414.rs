//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 414/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk414(t142: f64, t1500: f64, t100: f64, t95: f64, t1235: f64, t103: f64, t1251: f64, t1: f64, t120: f64, t485: f64, t119: f64, t155: f64, t481: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1501 = t1500 * t142;
    let t1503 = t95 * t100;
    let t1508 = param_hyb_omega_0 * t1235;
    let t1509 = t1508 * t103;
    let t1511 = 0.32478055555555555555e0_f64 * t1509 * t1251;
    let t1513 = t485 * t120 * t1;
    let t1515 = t119 * t155 * t481;
    (t1501, t1503, t1508, t1509, t1511, t1513, t1515)
}
