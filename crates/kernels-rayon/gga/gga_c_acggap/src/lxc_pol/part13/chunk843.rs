//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 843/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk843(t1164: f64, t4847: f64, t1298: f64, t467: f64, t1410: f64, t407: f64, t406: f64, t6263: f64, t1454: f64, t322: f64, t513: f64, t943: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17972 = t1164 * t4847;
    let t19409 = t1298 * t467;
    let t19834 = t407 * t1410;
    let t20138 = t6263 * t406;
    let t20311 = t1454 * t322;
    let t20432 = t513 * t943;
    (t17972, t19409, t19834, t20138, t20311, t20432)
}
