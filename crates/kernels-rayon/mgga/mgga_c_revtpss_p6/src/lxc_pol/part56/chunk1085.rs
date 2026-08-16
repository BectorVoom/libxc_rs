//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1085/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1085(t33612: f64, t644: f64, t8621: f64, t640: f64, t7705: f64, t1469: f64, t606: f64, t8441: f64, t32143: f64, t4186: f64, t37: f64, t2247: f64) -> (f64, f64, f64, f64, f64) {
    let t125290 = t8621 * t33612 * t644;
    let t125298 = t8621 * t7705 * t640;
    let t125305 = t8621 * t8441 * t606 * t1469;
    let t125309 = t8621 * t32143 * t4186;
    let t125312 = t37 * t606;
    let t125313 = t2247 * t125312;
    (t125290, t125298, t125305, t125309, t125313)
}
