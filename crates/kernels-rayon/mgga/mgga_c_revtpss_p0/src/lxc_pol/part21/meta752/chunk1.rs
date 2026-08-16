//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2631/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2631(t33: f64, t5585: f64, t588: f64, t1113: f64, t1348: f64, t13701: f64, t13704: f64, t1711: f64, t2: f64, t22: f64, t3881: f64, t46328: f64, t48192: f64, t48195: f64, t48201: f64, t48204: f64, t5582: f64, t580: f64, t9351: f64, t9357: f64, t9617: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t48417 = 16.0_f64 * t5585 * t588;
    let t48419 = piecewise3(t34, 0.0_f64, -56.0_f64 / 81.0_f64 * t46328 * t1711 * t9351 - 16.0_f64 / 9.0_f64 * t9617 * t2 * t48192 + 8.0_f64 / 9.0_f64 * t13701 * t48195 + 4.0_f64 / 3.0_f64 * t3881 * t580 * t1113 - 4.0_f64 * t13704 * t48201 + 4.0_f64 / 3.0_f64 * t13704 * t48204 - 2.0_f64 / 9.0_f64 * t5582 * t9357 + 8.0_f64 * t1348 * t22 - t48417);
    t48419
}
