//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2616/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2616(t33: f64, t3842: f64, t580: f64, t1113: f64, t3351: f64, t22: f64, t5560: f64, t588: f64, t13565: f64, t13568: f64, t1711: f64, t2: f64, t3841: f64, t47040: f64, t516: f64, t5557: f64, t9350: f64, t9351: f64, t9357: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t48192 = t580 * t3842;
    let t48195 = t1113 * t3351;
    let t48201 = t22 * t1113;
    let t48204 = t580 * t3351;
    let t48212 = 32.0_f64 * t5560 * t588;
    let t48214 = piecewise3(t34, 0.0_f64, 40.0_f64 / 81.0_f64 * t47040 * t1711 * t9351 + 16.0_f64 / 9.0_f64 * t9350 * t2 * t48192 - 8.0_f64 / 9.0_f64 * t13565 * t48195 - 8.0_f64 / 3.0_f64 * t3841 * t580 * t1113 + 8.0_f64 * t13568 * t48201 - 8.0_f64 / 3.0_f64 * t13568 * t48204 + 4.0_f64 / 9.0_f64 * t5557 * t9357 + 16.0_f64 * t516 * t22 - t48212);
    (t48192, t48195, t48201, t48204, t48214)
}
