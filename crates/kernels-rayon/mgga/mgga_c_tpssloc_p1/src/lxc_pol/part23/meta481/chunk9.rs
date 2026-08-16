//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1448/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1448(t11678: f64, t1227: f64, t15507: f64, t15654: f64, t1653: f64, t1734: f64, t1737: f64, t1748: f64, t19033: f64, t22275: f64, t22301: f64, t3578: f64, t4582: f64, t4972: f64, t53087: f64, t6211: f64, t65444: f64, t65464: f64, t72161: f64, t72181: f64, t72183: f64, t72389: f64, t72398: f64, t72967: f64, t77606: f64, t77621: f64) -> f64 {
    let t78689 = t15507 * t22275 / 48.0_f64 - t72161 / 36.0_f64 + t65444 / 216.0_f64 - t1227 * t4582 * t4972 * t77621 / 576.0_f64 + 5.0_f64 / 384.0_f64 * t1227 * t4582 * t15654 * t77606 + t72181 / 384.0_f64 - 209.0_f64 / 648.0_f64 * t72389 * t1737 + 209.0_f64 / 972.0_f64 * t72398 * t1748 - 19.0_f64 / 216.0_f64 * t19033 * t6211 - t72183 / 576.0_f64 - t53087 * t22301 / 144.0_f64 + 19.0_f64 / 144.0_f64 * t72967 * t1737 - t11678 * t3578 * t65464 * t1653 * t1734 / 192.0_f64;
    t78689
}
