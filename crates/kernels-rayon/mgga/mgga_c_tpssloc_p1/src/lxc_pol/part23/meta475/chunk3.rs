//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1423/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1423(t44027: f64, t50846: f64, t71470: f64, t71472: f64, t71474: f64, t78026: f64, t78029: f64, t78033: f64, t78037: f64, t78041: f64, t78045: f64, t78049: f64, t78078: f64, t78080: f64) -> f64 {
    let t78177 = -0.97370864197530864199e0_f64 * t50846 - 0.97370864197530864196e-1_f64 * t71470 + 0.43816888888888888888e0_f64 * t71472 - 0.13145066666666666666e1_f64 * t71474 + t44027 - 0.28483875e1_f64 * t78026 + 0.1151859375e0_f64 * t78029 - 0.79724444444444444444e0_f64 * t78033 + 0.19931111111111111111e1_f64 * t78037 - 0.71752000000000000001e1_f64 * t78041 + 0.107628e2_f64 * t78045 + 0.23917333333333333333e1_f64 * t78049 + 0.3071625e0_f64 * t78078 - 0.3560484375e1_f64 * t78080;
    t78177
}
