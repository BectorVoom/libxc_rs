//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1134/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1134(t22705: f64, t28130: f64, t81228: f64, t22704: f64, t28134: f64, t80798: f64, t22892: f64, t22893: f64, t28148: f64, t22751: f64, t28149: f64, t28139: f64) -> (f64, f64, f64, f64, f64) {
    let t97043 = t81228 * t22705 * t28130;
    let t97049 = t22704 * t80798 * t28134;
    let t97070 = t22892 * t22893 * t28148;
    let t97095 = t22751 * t28149;
    let t97108 = t22751 * t28139;
    (t97043, t97049, t97070, t97095, t97108)
}
