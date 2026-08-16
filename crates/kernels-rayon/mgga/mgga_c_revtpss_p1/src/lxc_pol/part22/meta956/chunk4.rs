//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3204/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3204(t10389: f64, t10398: f64, t13312: f64, t13368: f64, t13378: f64, t13396: f64, t18281: f64, t21784: f64, t21789: f64, t21794: f64, t21799: f64, t2251: f64, t2258: f64, t2299: f64, t2306: f64, t4227: f64, t4232: f64, t46001: f64, t46014: f64, t5819: f64, t5825: f64, t606: f64, t60717: f64, t60754: f64, t633: f64, t637: f64) -> f64 {
    let t60778 = 3640.0_f64 / 81.0_f64 * t46001 * t5819 * t2251 - 1120.0_f64 / 27.0_f64 * t13368 * t13396 - 280.0_f64 / 27.0_f64 * t21784 * t2258 + 56.0_f64 / 9.0_f64 * t2299 * t60717 + 56.0_f64 / 9.0_f64 * t4227 * t13312 - 280.0_f64 / 27.0_f64 * t10389 * t5825 * t2251 + 56.0_f64 / 9.0_f64 * t2299 * t18281 * t606 + 28.0_f64 / 9.0_f64 * t21789 * t2258 - 4.0_f64 / 3.0_f64 * t633 * t60754 + 3640.0_f64 / 81.0_f64 * t46014 * t5819 * t2251 + 1120.0_f64 / 27.0_f64 * t13378 * t13396 + 280.0_f64 / 27.0_f64 * t21794 * t2258 + 56.0_f64 / 9.0_f64 * t2306 * t60717 + 56.0_f64 / 9.0_f64 * t4232 * t13312 + 280.0_f64 / 27.0_f64 * t10398 * t5825 * t2251 + 56.0_f64 / 9.0_f64 * t2306 * t18281 * t606 + 28.0_f64 / 9.0_f64 * t21799 * t2258 + 4.0_f64 / 3.0_f64 * t637 * t60754;
    t60778
}
