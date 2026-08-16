//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1694/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1694(t21768: f64, t38: f64, t10389: f64, t5819: f64, t2299: f64, t5825: f64, t10398: f64, t2306: f64, t18281: f64, t4186: f64, t4227: f64, t4232: f64, t606: f64, t633: f64, t637: f64) -> (f64, f64) {
    let t21769 = t38 * t21768;
    let t21784 = t10389 * t5819;
    let t21789 = t2299 * t5825;
    let t21794 = t10398 * t5819;
    let t21799 = t2306 * t5825;
    let t21804 = -280.0_f64 / 27.0_f64 * t21784 * t606 + 56.0_f64 / 9.0_f64 * t4227 * t4186 + 28.0_f64 / 9.0_f64 * t21789 * t606 - 4.0_f64 / 3.0_f64 * t633 * t18281 + 280.0_f64 / 27.0_f64 * t21794 * t606 + 56.0_f64 / 9.0_f64 * t4232 * t4186 + 28.0_f64 / 9.0_f64 * t21799 * t606 + 4.0_f64 / 3.0_f64 * t637 * t18281;
    (t21769, t21804)
}
