//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1119/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1119(t2060: f64, t361: f64, t9704: f64, t1165: f64, t5969: f64, t604: f64, t7493: f64, t1992: f64, t30692: f64, t7842: f64, t9587: f64, t7839: f64, t9601: f64) -> (f64, f64, f64, f64) {
    let t39337 = t2060 * t361 * t9704;
    let t39343 = t7493 * t1165 * t604 * t5969;
    let t39356 = t30692 * t7842 * t1992 * t9587;
    let t39358 = t7839 * t9601;
    (t39337, t39343, t39356, t39358)
}
