//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2615/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2615(t30: f64, t3834: f64, t580: f64, t2257: f64, t605: f64, t22: f64, t5552: f64, t588: f64, t13550: f64, t13553: f64, t1468: f64, t2: f64, t3833: f64, t47025: f64, t513: f64, t5549: f64, t9335: f64, t9336: f64, t9344: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t48165 = t580 * t3834;
    let t48168 = t605 * t2257;
    let t48174 = t22 * t605;
    let t48177 = t580 * t2257;
    let t48185 = 32.0_f64 * t5552 * t588;
    let t48187 = piecewise3(t31, 0.0_f64, 40.0_f64 / 81.0_f64 * t47025 * t1468 * t9336 - 16.0_f64 / 9.0_f64 * t9335 * t2 * t48165 - 8.0_f64 / 9.0_f64 * t13550 * t48168 + 8.0_f64 / 3.0_f64 * t3833 * t580 * t605 - 8.0_f64 * t13553 * t48174 + 8.0_f64 / 3.0_f64 * t13553 * t48177 + 4.0_f64 / 9.0_f64 * t5549 * t9344 - 16.0_f64 * t513 * t22 + t48185);
    (t48165, t48168, t48174, t48177, t48187)
}
