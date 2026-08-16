//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1442/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1442(t30: f64, t1448: f64, t4144: f64, t4146: f64, t565: f64, t1333: f64, t3860: f64, t4147: f64, t513: f64, t3874: f64, t605: f64, t1344: f64, t2257: f64, t9336: f64, t9344: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t9590 = t4144 * t1448;
    let t9593 = 1.0_f64 / t4146 / t565;
    let t9597 = t3860 * t1333;
    let t9598 = 36.0_f64 * t9597;
    let t9599 = t4144 * t4147;
    let t9603 = t30 * t30;
    let t9605 = 1.0_f64 / t513 / t9603;
    let t9608 = t3874 * t605;
    let t9614 = piecewise3(t31, 0.0_f64, 8.0_f64 / 27.0_f64 * t9605 * t9336 - 2.0_f64 / 3.0_f64 * t9608 * t2257 + 2.0_f64 / 3.0_f64 * t1344 * t9344);
    (t9590, t9593, t9597, t9598, t9599, t9603, t9605, t9614)
}
