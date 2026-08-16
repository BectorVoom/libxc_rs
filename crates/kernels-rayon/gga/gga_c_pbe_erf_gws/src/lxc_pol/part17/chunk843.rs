//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 843/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk843(t1816: f64, t7106: f64, t5211: f64, t1044: f64, t5212: f64, t1811: f64, t108: f64, t210: f64, t267: f64, t1791: f64, t641: f64, t1018: f64, t1672: f64) -> (f64, f64, f64, f64, f64) {
    let t7107 = t7106 * t1816;
    let t7109 = 16.0_f64 / 45.0_f64 * t5211 * t7107;
    let t7110 = t5212 * t1044;
    let t7111 = t7110 * t1811;
    let t7113 = 16.0_f64 / 45.0_f64 * t5211 * t7111;
    let t7114 = t210 * t108;
    let t7115 = t7114 * t267;
    let t7116 = t641 * t1791;
    let t7117 = t7116 * t1044;
    let t7118 = t7117 * t1816;
    let t7120 = 16.0_f64 / 45.0_f64 * t7115 * t7118;
    let t7121 = t1672 * t1018;
    (t7109, t7113, t7115, t7120, t7121)
}
