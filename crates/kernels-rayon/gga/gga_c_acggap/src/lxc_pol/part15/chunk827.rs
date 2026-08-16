//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 827/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk827(t1181: f64, t9592: f64, t2068: f64, t1165: f64, t604: f64, t9587: f64, t7337: f64, t6841: f64, t7351: f64, t1854: f64, t7564: f64, t1750: f64, t7561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9593 = t1181 * t9592;
    let t9594 = t2068 * t9593;
    let t9597 = t1165 * t604 * t9587;
    let t9598 = t7337 * t9597;
    let t9601 = t1165 * t7351 * t6841;
    let t9602 = t2068 * t9601;
    let t9607 = t7351 * t1854;
    let t9608 = t1181 * t9607;
    let t9609 = t7564 * t9608;
    let t9611 = t7561 * t1750;
    (t9593, t9594, t9597, t9598, t9601, t9602, t9607, t9608, t9609, t9611)
}
