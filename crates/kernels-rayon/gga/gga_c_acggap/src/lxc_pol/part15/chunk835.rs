//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 835/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk835(t1181: f64, t9719: f64, t7575: f64, t1089: f64, t1459: f64, t9563: f64, t598: f64, t142: f64, t1866: f64, t7436: f64, t7815: f64, t2030: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9720 = t1181 * t9719;
    let t9721 = t7575 * t9720;
    let t9724 = t1089 * t1459 * t9563;
    let t9725 = t598 * t9724;
    let t9727 = t142 * t1866;
    let t9728 = t7436 * t9727;
    let t9730 = t7815 * t1866;
    let t9731 = t2030 * t9730;
    (t9720, t9721, t9724, t9725, t9727, t9728, t9730, t9731)
}
