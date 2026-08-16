//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2482/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2482(t1023: f64, t14218: f64, t14508: f64, t17673: f64, t17701: f64, t17734: f64, t21138: f64, t21597: f64, t3070: f64, t3071: f64, t3114: f64, t42388: f64, t42752: f64, t4650: f64, t48570: f64, t48611: f64, t49853: f64, t49872: f64, t49934: f64, t5681: f64, t62306: f64, t69935: f64) -> f64 {
    let t70623 = -t49853 + t14508 * t17734 / 256.0_f64 + 3.0_f64 / 512.0_f64 * t48570 * t17673 - t49934 * t17701 / 1536.0_f64 + t42752 / 15552.0_f64 - t49872 - t62306 / 2304.0_f64 + t3070 * t3071 * t21138 * t1023 / 768.0_f64 + t3114 * t21597 / 3072.0_f64 - t3070 * t3071 * t5681 * t4650 / 768.0_f64 + 3.0_f64 / 512.0_f64 * t42388 * t48611 * t69935 * t14218;
    t70623
}
