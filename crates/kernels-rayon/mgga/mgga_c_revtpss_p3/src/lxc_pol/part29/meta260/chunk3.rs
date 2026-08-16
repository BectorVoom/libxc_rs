//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1067/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1067(t234: f64, t243: f64, t7028: f64, t807: f64, t1945: f64, t786: f64, t817: f64, t64: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t7030 = t234 * t7028 * t243;
    let t7031 = t807 * t7030;
    let t7033 = t786 * t1945;
    let t7034 = t7033 * t817;
    let t7036 = t822 * t64;
    (t7030, t7031, t7033, t7034, t7036)
}
