//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 266/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk266(t1113: f64, t384: f64, t1047: f64, t1053: f64, t1075: f64, t329: f64, t334: f64) -> (f64, f64, f64, f64, f64) {
    let t1114 = t384 * t1113;
    let t1124 = 0.1141e1_f64 * t1047;
    let t1126 = 0.2445e0_f64 * t1053;
    let t1130 = 0.12225e0_f64 * t1075;
    let t1137 = t329 * t334;
    (t1114, t1124, t1126, t1130, t1137)
}
