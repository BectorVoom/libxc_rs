//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1325/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1325(t15904: f64, t22574: f64, t31035: f64, t12303: f64, t24995: f64, t8945: f64, t1266: f64, t22479: f64, t652: f64, t1874: f64, t45637: f64, t12458: f64, t40611: f64) -> (f64, f64, f64, f64, f64) {
    let t83684 = 18.0_f64 * t22574 * t31035 * t15904;
    let t83687 = 18.0_f64 * t24995 * t8945 * t12303;
    let t83692 = 6.0_f64 * t652 * t1266 * t22479;
    let t83694 = 6.0_f64 * t45637 * t1874;
    let t83695 = t40611 * t12458;
    (t83684, t83687, t83692, t83694, t83695)
}
