//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 608/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk608(t2606: f64, t2670: f64, t2752: f64, t2829: f64, t1076: f64, t153: f64, t542: f64, t1220: f64, t1278: f64, t1288: f64, t1296: f64, t1328: f64, t1335: f64, t1338: f64, t1426: f64, t1431: f64, t1450: f64, t2449: f64, t2476: f64, t2508: f64) -> (f64, f64, f64) {
    let t2831 = t2606 + t2670 + t2752 + t2829;
    let t2837 = t153 * t542 * t1076;
    let t2839 = t1220 + t1328 + t1335 - t1338 + t1426 - t2449 + t1450 - t1278 + t1288 + t1296 + t2476 - t2508 - t1431;
    (t2831, t2837, t2839)
}
