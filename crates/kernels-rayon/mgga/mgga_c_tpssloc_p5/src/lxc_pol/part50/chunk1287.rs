//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1287/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1287(t31280: f64, t33185: f64, t23877: f64, t7467: f64, t7769: f64, t83980: f64, t20173: f64, t33193: f64, t3941: f64, t4072: f64, t8326: f64, t7015: f64, t86647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120792 = 54.0_f64 * t33185 * t31280;
    let t120793 = t23877 * t7467;
    let t120795 = t83980 * t7769;
    let t120800 = 27.0_f64 * t20173 * t33193;
    let t120803 = 27.0_f64 * t3941 * t8326 * t4072;
    let t120804 = t86647 * t7015;
    (t120792, t120793, t120795, t120800, t120803, t120804)
}
