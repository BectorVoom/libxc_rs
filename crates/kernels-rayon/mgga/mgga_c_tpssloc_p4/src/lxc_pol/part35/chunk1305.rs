//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1305/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1305(t562: f64, t6330: f64, t22704: f64, t22705: f64, t28163: f64, t28130: f64, t81228: f64, t28134: f64, t80798: f64, t22892: f64, t22893: f64, t28148: f64) -> (f64, f64, f64, f64, f64) {
    let t97011 = t562 * t6330;
    let t97026 = t22704 * t22705 * t28163;
    let t97043 = t81228 * t22705 * t28130;
    let t97049 = t22704 * t80798 * t28134;
    let t97070 = t22892 * t22893 * t28148;
    (t97011, t97026, t97043, t97049, t97070)
}
