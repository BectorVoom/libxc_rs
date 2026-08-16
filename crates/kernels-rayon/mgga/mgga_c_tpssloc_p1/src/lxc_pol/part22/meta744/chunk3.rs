//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2471/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2471(t1023: f64, t10390: f64, t1041: f64, t17637: f64, t17643: f64, t21134: f64, t21403: f64, t21532: f64, t21574: f64, t3070: f64, t3071: f64, t42397: f64, t42483: f64, t42508: f64, t4582: f64, t4583: f64, t4644: f64, t4650: f64, t48607: f64, t49854: f64, t5685: f64, t69643: f64, t70316: f64, t70321: f64, t70330: f64, t884: f64) -> f64 {
    let t70335 = t42508 * t21532 / 288.0_f64 + t42483 * t3071 * t21403 * t884 / 4608.0_f64 + t3070 * t3071 * t5685 * t4650 / 1536.0_f64 + t3070 * t3071 * t21134 * t1023 / 4608.0_f64 + t10390 * t21574 / 1536.0_f64 + 5.0_f64 / 1728.0_f64 * t48607 * t42397 * t69643 - t1041 * t4582 * t4583 * t70316 / 768.0_f64 - t1041 * t4582 * t4583 * t70321 / 768.0_f64 - t4644 * t17637 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t4644 * t17643 - 5.0_f64 / 432.0_f64 * t1041 * t4582 * t49854 * t70330;
    t70335
}
