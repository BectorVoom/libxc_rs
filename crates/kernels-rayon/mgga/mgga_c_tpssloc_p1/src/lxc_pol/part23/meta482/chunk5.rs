//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1456/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1456(t44249: f64, t50846: f64, t71470: f64, t71472: f64, t71474: f64, t78026: f64, t78029: f64, t78033: f64, t78037: f64, t78041: f64, t78045: f64, t78049: f64, t78078: f64, t78080: f64) -> f64 {
    let t78839 = -0.12349037037037037037e1_f64 * t50846 - 0.12349037037037037037e0_f64 * t71470 + 0.55570666666666666668e0_f64 * t71472 - 0.166712e1_f64 * t71474 + t44249 - 0.52945875e1_f64 * t78026 + 0.2366859375e0_f64 * t78029 - 0.13772666666666666667e1_f64 * t78033 + 0.34431666666666666667e1_f64 * t78037 - 0.123954e2_f64 * t78041 + 0.185931e2_f64 * t78045 + 0.41318e1_f64 * t78049 + 0.6311625e0_f64 * t78078 - 0.6618234375e1_f64 * t78080;
    t78839
}
