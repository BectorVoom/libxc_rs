//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1372/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1372(t1433: f64, t5389: f64, t72: f64, t3953: f64, t5399: f64, t20201: f64, t79: f64, t5445: f64, t1865: f64, t22544: f64, t26051: f64, t26084: f64, t27961: f64, t27976: f64, t27979: f64, t6490: f64, t7432: f64, t7442: f64, t7446: f64, t83830: f64, t90192: f64, t90330: f64, t96547: f64) -> f64 {
    let t106826 = t72 * t1433 * t5389;
    let t106829 = t3953 * t5399;
    let t106836 = t72 * t79 * t20201;
    let t106842 = t72 * t1433 * t5445;
    let t106847 = -15.0_f64 * t90330 * t27961 - 15.0_f64 * t90192 * t27961 - 15.0_f64 * t22544 * t106826 + t106829 * t1865 + 5.0_f64 / 2.0_f64 * t26084 * t27976 - 5.0_f64 * t96547 * t7432 + 35.0_f64 * t83830 * t106836 + t27979 * t7442 + t27979 * t7446 + 5.0_f64 / 2.0_f64 * t6490 * t106842 + 5.0_f64 / 2.0_f64 * t26051 * t27976;
    t106847
}
