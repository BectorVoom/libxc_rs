//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 985/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk985(t109: f64, t22473: f64, t26129: f64, t4067: f64, t6530: f64, t22469: f64, t22471: f64, t26127: f64) -> f64 {
    let t110 = 1.0_f64 < t109;
    let t26130 = t22473 * t26129;
    let t26132 = t6530 * t4067;
    let t26135 = piecewise3(t110, 0.0_f64, t22469 + t22471 / 3.0_f64 + t26127 / 3.0_f64 + t26130 / 4.0_f64 - t26132 / 8.0_f64);
    t26135
}
