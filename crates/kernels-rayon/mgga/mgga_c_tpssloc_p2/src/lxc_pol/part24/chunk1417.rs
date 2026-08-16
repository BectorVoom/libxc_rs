//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1417/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1417(t1266: f64, t22479: f64, t652: f64, t1874: f64, t45637: f64, t12458: f64, t40611: f64, t1983: f64, t2019: f64, t2235: f64, t2244: f64, t71: f64, t9338: f64) -> (f64, f64, f64, f64, f64) {
    let t83692 = 6.0_f64 * t652 * t1266 * t22479;
    let t83694 = 6.0_f64 * t45637 * t1874;
    let t83695 = t40611 * t12458;
    let t83698 = 6.0_f64 * t1983 * t2019 * t83695;
    let t83699 = t2235 * t2244;
    let t83706 = t71 * t9338;
    (t83692, t83694, t83698, t83699, t83706)
}
