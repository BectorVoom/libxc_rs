//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 887/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk887(t1181: f64, t3754: f64, t604: f64, t7426: f64, t1170: f64, t2066: f64, t592: f64, t7777: f64, t2070: f64, t1165: f64, t3759: f64, t7351: f64) -> (f64, f64, f64, f64) {
    let t30452 = t7426 * t1181 * t604 * t3754;
    let t30456 = t1170 * t592 * t7777 * t2066;
    let t30457 = t30456 * t2070;
    let t30463 = t7426 * t1165 * t7351 * t3759;
    (t30452, t30456, t30457, t30463)
}
