//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2478/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2478(t1041: f64, t10949: f64, t14211: f64, t21487: f64, t21538: f64, t21562: f64, t2960: f64, t3130: f64, t4582: f64, t4588: f64, t4596: f64, t4600: f64, t61736: f64, t61739: f64, t62091: f64, t62137: f64, t62148: f64, t62150: f64, t62152: f64, t70458: f64) -> f64 {
    let t70481 = 5.0_f64 / 13824.0_f64 * t1041 * t4582 * t4588 * t70458 + 2.0_f64 / 27.0_f64 * t2960 * t21538 - t2960 * t21562 / 18.0_f64 + t62137 / 3456.0_f64 - t62148 / 2304.0_f64 - t62150 / 432.0_f64 + t62152 / 768.0_f64 + t10949 * t21487 / 512.0_f64 + t3130 * t4582 * t62091 * t14211 / 512.0_f64 + t61736 * t4596 / 512.0_f64 - t61739 * t4600 / 1024.0_f64;
    t70481
}
