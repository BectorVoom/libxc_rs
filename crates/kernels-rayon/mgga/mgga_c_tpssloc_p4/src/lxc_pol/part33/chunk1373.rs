//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1373/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1373(t20288: f64, t72: f64, t79: f64, t5398: f64, t20218: f64, t605: f64, t1410: f64, t19299: f64, t1865: f64, t26051: f64, t27966: f64, t27972: f64, t27982: f64, t6490: f64, t7432: f64, t7442: f64, t7446: f64, t96529: f64, t96532: f64, t96538: f64, t96551: f64) -> f64 {
    let t106849 = t72 * t79 * t20288;
    let t106853 = t72 * t79 * t5398;
    let t106855 = t605 * t20218;
    let t106862 = t19299 * t1410;
    let t106874 = 5.0_f64 / 6.0_f64 * t6490 * t106849 + t96551 * t106853 + t106855 * t1865 / 3.0_f64 + t27982 * t7442 + t27982 * t7446 + 5.0_f64 / 2.0_f64 * t96532 * t7432 + t106862 * t1865 + 5.0_f64 * t96538 * t7432 + 2.0_f64 * t27966 * t7442 + 5.0_f64 * t26051 * t27972 + 2.0_f64 * t27966 * t7446 + 5.0_f64 / 2.0_f64 * t96529 * t7432;
    t106874
}
