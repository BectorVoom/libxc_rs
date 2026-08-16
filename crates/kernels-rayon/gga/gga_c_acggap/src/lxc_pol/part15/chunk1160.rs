//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1160/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1160(t1165: f64, t39753: f64, t604: f64, t7337: f64, t31142: f64, t9727: f64, t2060: f64, t361: f64, t9733: f64, t7450: f64, t9659: f64, t13287: f64, t31195: f64, t38861: f64) -> (f64, f64, f64, f64, f64) {
    let t40080 = t7337 * t1165 * t604 * t39753;
    let t40083 = t31142 * t9727;
    let t40086 = t2060 * t361 * t9733;
    let t40089 = t7450 * t361 * t9659;
    let t40092 = t31195 * t13287 * t38861;
    (t40080, t40083, t40086, t40089, t40092)
}
