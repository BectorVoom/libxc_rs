//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1780/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1780(t3167: f64, t7120: f64, t1033: f64, t3173: f64, t7122: f64, t1007: f64, t7106: f64, t1968: f64, t3080: f64, t7105: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25525 = t7120 * t3167;
    let t25526 = t1033 * t25525;
    let t25529 = t7122 * t3173;
    let t25535 = t7106 * t1007;
    let t25538 = t1968 * t3080 / 432.0_f64;
    let t25539 = t7105 * t800;
    (t25525, t25526, t25529, t25535, t25538, t25539)
}
