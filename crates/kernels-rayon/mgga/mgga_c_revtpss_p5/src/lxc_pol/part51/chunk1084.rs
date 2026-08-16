//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1084/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1084(t640: f64, t7705: f64, t8621: f64, t1469: f64, t606: f64, t8441: f64, t32143: f64, t4186: f64, t37: f64, t2247: f64, t8442: f64, t33620: f64, t644: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125298 = t8621 * t7705 * t640;
    let t125305 = t8621 * t8441 * t606 * t1469;
    let t125309 = t8621 * t32143 * t4186;
    let t125312 = t37 * t606;
    let t125313 = t2247 * t125312;
    let t125314 = t8442 * t1469;
    let t125319 = t8621 * t33620 * t644;
    (t125298, t125305, t125309, t125313, t125314, t125319)
}
