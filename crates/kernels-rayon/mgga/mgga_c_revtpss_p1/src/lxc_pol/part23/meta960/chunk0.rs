//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3233/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3233(t22648: f64, t602: f64, t13368: f64, t13371: f64, t13378: f64, t13381: f64, t18281: f64, t19680: f64, t21784: f64, t21794: f64, t22671: f64, t22688: f64, t2299: f64, t2306: f64, t4186: f64, t4227: f64, t4232: f64, t46001: f64, t46014: f64, t5825: f64, t606: f64, t633: f64, t637: f64, t76397: f64) -> (f64, f64) {
    let t85037 = t22648 * t602;
    let t85125 = 3640.0_f64 / 81.0_f64 * t46001 * t22688 * t606 - 280.0_f64 / 9.0_f64 * t21784 * t4186 - 280.0_f64 / 9.0_f64 * t13368 * t19680 + 28.0_f64 / 3.0_f64 * t13371 * t5825 + 28.0_f64 / 3.0_f64 * t4227 * t18281 + 28.0_f64 / 9.0_f64 * t2299 * t22671 * t606 - 4.0_f64 / 3.0_f64 * t633 * t76397 + 3640.0_f64 / 81.0_f64 * t46014 * t22688 * t606 + 280.0_f64 / 9.0_f64 * t21794 * t4186 + 280.0_f64 / 9.0_f64 * t13378 * t19680 + 28.0_f64 / 3.0_f64 * t13381 * t5825 + 28.0_f64 / 3.0_f64 * t4232 * t18281 + 28.0_f64 / 9.0_f64 * t2306 * t22671 * t606 + 4.0_f64 / 3.0_f64 * t637 * t76397;
    (t85037, t85125)
}
