//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3237/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3237(t1480: f64, t21754: f64, t21762: f64, t21765: f64, t22689: f64, t22695: f64, t22700: f64, t4186: f64, t4214: f64, t44: f64, t46090: f64, t48: f64, t56: f64, t5843: f64, t60: f64, t60308: f64, t60311: f64, t614: f64, t620: f64, t76397: f64, t77513: f64) -> f64 {
    let t85255 = -5.0_f64 / 36.0_f64 * t60308 * t77513 + 5.0_f64 / 36.0_f64 * t60311 * t77513 - t46090 + 10.0_f64 / 81.0_f64 * t614 * t22689 - 20.0_f64 / 9.0_f64 * t614 * t22695 + 5.0_f64 / 6.0_f64 * t44 * t48 * t76397 + 3080.0_f64 / 81.0_f64 * t22700 * t620 - 220.0_f64 / 9.0_f64 * t5843 * t4214 + 20.0_f64 / 3.0_f64 * t1480 * t21765 - 5.0_f64 / 6.0_f64 * t56 * t60 * t76397 - 20.0_f64 / 9.0_f64 * t1480 * t21762 + 5.0_f64 / 36.0_f64 * t56 * t21754 * t4186;
    t85255
}
