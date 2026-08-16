//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1495/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1495(t105880: f64, t117218: f64, t117544: f64, t117932: f64, t118374: f64, t1509: f64, t21864: f64, t31035: f64, t31149: f64, t31287: f64, t31420: f64, t31433: f64, t31439: f64, t31443: f64, t4287: f64, t5907: f64, t5911: f64, t5915: f64, t661: f64, t665: f64, t8258: f64, t8267: f64, t8311: f64, t8315: f64) -> f64 {
    let t118728 = -25.0_f64 / 18.0_f64 * t8258 * t31433 * t31420 + 5.0_f64 / 6.0_f64 * t8258 * t8315 * t4287 * t1509 - 5.0_f64 / 6.0_f64 * t117544 * t8315 * t118374 - 3.0_f64 / 4.0_f64 * t31035 * t8311 * t105880 + 5.0_f64 / 12.0_f64 * t8258 * t8315 * t5915 * t661 - 25.0_f64 / 18.0_f64 * t8258 * t31433 * t31439 + 25.0_f64 / 54.0_f64 * t8267 * t117932 * t31443 + 5.0_f64 / 18.0_f64 * t8258 * t31149 * t5907 * t665 + 5.0_f64 / 108.0_f64 * t8267 * t117218 * t5907 * t661 + 5.0_f64 / 18.0_f64 * t31287 * t31149 * t21864 + 5.0_f64 / 12.0_f64 * t8258 * t8315 * t5911 * t665;
    t118728
}
