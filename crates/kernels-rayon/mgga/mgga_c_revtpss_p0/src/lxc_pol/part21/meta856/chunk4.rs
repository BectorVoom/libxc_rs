//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3251/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3251(t10326: f64, t10356: f64, t10389: f64, t10398: f64, t11231: f64, t13312: f64, t13368: f64, t13371: f64, t13378: f64, t13381: f64, t1469: f64, t2251: f64, t2258: f64, t2299: f64, t2306: f64, t4186: f64, t4227: f64, t4232: f64, t46001: f64, t46014: f64, t49889: f64, t606: f64, t633: f64, t637: f64) -> f64 {
    let t60479 = 3640.0_f64 / 81.0_f64 * t46001 * t1469 * t10356 - 280.0_f64 / 9.0_f64 * t10389 * t4186 * t2251 - 280.0_f64 / 9.0_f64 * t13368 * t11231 + 28.0_f64 / 3.0_f64 * t2299 * t13312 * t606 + 28.0_f64 / 3.0_f64 * t13371 * t2258 + 28.0_f64 / 9.0_f64 * t4227 * t10326 - 4.0_f64 / 3.0_f64 * t633 * t49889 + 3640.0_f64 / 81.0_f64 * t46014 * t1469 * t10356 + 280.0_f64 / 9.0_f64 * t10398 * t4186 * t2251 + 280.0_f64 / 9.0_f64 * t13378 * t11231 + 28.0_f64 / 3.0_f64 * t2306 * t13312 * t606 + 28.0_f64 / 3.0_f64 * t13381 * t2258 + 28.0_f64 / 9.0_f64 * t4232 * t10326 + 4.0_f64 / 3.0_f64 * t637 * t49889;
    t60479
}
