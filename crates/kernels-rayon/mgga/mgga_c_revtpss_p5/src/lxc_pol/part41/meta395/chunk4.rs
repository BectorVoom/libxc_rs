//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1340/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1340(t20567: f64, t448: f64, t17092: f64, t5068: f64, t16840: f64, t5109: f64, t1149: f64, t6439: f64, t3433: f64, t1733: f64, t5104: f64, t3384: f64) -> (f64, f64, f64, f64, f64) {
    let t20568 = t20567 * t448;
    let t20571 = 4.0_f64 * t17092 * t5068;
    let t20573 = 0.32163958997385070134e2_f64 * t16840 * t5109;
    let t20574 = t6439 * t1149;
    let t20576 = 6.0_f64 * t3433 * t20574;
    let t20577 = t1733 * t5104;
    let t20579 = 4.0_f64 * t3384 * t20577;
    (t20568, t20571, t20573, t20576, t20579)
}
