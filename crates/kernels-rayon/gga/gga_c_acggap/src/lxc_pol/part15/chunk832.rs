//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 832/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk832(t1856: f64, t2001: f64, t1426: f64, t368: f64, t9536: f64, t598: f64, t1772: f64, t6: f64, t422: f64, t599: f64, t5679: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9677 = t2001 * t1856;
    let t9681 = t1426 * t368 * t9536;
    let t9682 = t598 * t9681;
    let t9685 = t6 * t1772;
    let t9687 = t422 * t9685 * t599;
    let t9688 = t598 * t9687;
    let t9691 = t422 * t5679 * t604;
    (t9677, t9681, t9682, t9685, t9687, t9688, t9691)
}
