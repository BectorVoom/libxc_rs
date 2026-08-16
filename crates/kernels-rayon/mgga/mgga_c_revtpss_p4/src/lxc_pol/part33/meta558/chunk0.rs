//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1950/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1950(t30031: f64, t7296: f64, t6846: f64, t7264: f64, t6880: f64, t7271: f64, t6856: f64, t6876: f64, t26017: f64, t6850: f64, t26028: f64, t6871: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30032 = t7296 * t30031;
    let t30035 = t7264 * t6846;
    let t30037 = t7271 * t6880;
    let t30039 = t7271 * t6856;
    let t30041 = t7264 * t6876;
    let t30043 = t26017 * t6850;
    let t30045 = t26028 * t6871;
    (t30032, t30035, t30037, t30039, t30041, t30043, t30045)
}
