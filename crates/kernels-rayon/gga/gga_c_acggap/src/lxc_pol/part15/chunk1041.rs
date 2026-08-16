//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1041/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1041(t157: f64, t309: f64, t463: f64, t694: f64, t9114: f64, t2407: f64, t469: f64, t301: f64, t11179: f64, t1679: f64, t467: f64, t11883: f64, t642: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36495 = t157 * t463 * t309;
    let t36684 = 6.0_f64 * t694 * t9114;
    let t36686 = t2407 * t469;
    let t36689 = 6.0_f64 * t694 * t36686 * t301;
    let t36715 = 2.0_f64 * t1679 * t11179 * t467;
    let t36729 = t642 * t11883;
    (t36495, t36684, t36686, t36689, t36715, t36729)
}
