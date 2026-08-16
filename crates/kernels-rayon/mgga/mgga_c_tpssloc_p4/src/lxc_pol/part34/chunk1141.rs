//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1141/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1141(t22779: f64, t28067: f64, t28060: f64, t22892: f64, t22893: f64, t28138: f64, t28116: f64, t81228: f64, t81326: f64, t6897: f64, t7700: f64, t90544: f64) -> (f64, f64, f64, f64, f64) {
    let t97444 = t22779 * t28067;
    let t97463 = t22779 * t28060;
    let t97494 = t22892 * t22893 * t28138;
    let t97503 = t81228 * t81326 * t28116;
    let t97509 = t6897 * t90544 * t7700;
    (t97444, t97463, t97494, t97503, t97509)
}
