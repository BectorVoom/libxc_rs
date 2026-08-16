//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1141/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1141(t1389: f64, t3964: f64, t92986: f64, t7028: f64, t9736: f64, t9737: f64, t27932: f64, t47300: f64, t26009: f64, t9802: f64, t26004: f64, t3961: f64) -> (f64, f64, f64, f64, f64) {
    let t94476 = t3964 * t92986 * t1389;
    let t94479 = t9736 * t7028 * t9737;
    let t94481 = t27932 * t47300;
    let t94483 = t9802 * t26009;
    let t94485 = t26004 * t3961;
    (t94476, t94479, t94481, t94483, t94485)
}
