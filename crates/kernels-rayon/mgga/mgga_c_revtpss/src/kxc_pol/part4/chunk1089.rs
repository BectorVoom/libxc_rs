//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1089/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1089(t10389: f64, t1469: f64, t2299: f64, t4186: f64, t10398: f64, t2306: f64, t13312: f64, t2251: f64, t2258: f64, t4227: f64, t4232: f64, t606: f64, t633: f64, t637: f64) -> f64 {
    let t13368 = t10389 * t1469;
    let t13371 = t2299 * t4186;
    let t13378 = t10398 * t1469;
    let t13381 = t2306 * t4186;
    let t13388 = -280.0_f64 / 27.0_f64 * t13368 * t2251 + 56.0_f64 / 9.0_f64 * t13371 * t606 + 28.0_f64 / 9.0_f64 * t4227 * t2258 - 4.0_f64 / 3.0_f64 * t633 * t13312 + 280.0_f64 / 27.0_f64 * t13378 * t2251 + 56.0_f64 / 9.0_f64 * t13381 * t606 + 28.0_f64 / 9.0_f64 * t4232 * t2258 + 4.0_f64 / 3.0_f64 * t637 * t13312;
    t13388
}
