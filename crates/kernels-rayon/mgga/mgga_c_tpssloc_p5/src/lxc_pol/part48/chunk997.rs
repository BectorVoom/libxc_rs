//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 997/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk997(t22674: f64, t31607: f64, t6897: f64, t1985: f64, t80707: f64, t8621: f64, t22633: f64, t22635: f64, t31549: f64, t3719: f64, t31550: f64, t81228: f64, t81326: f64) -> (f64, f64, f64, f64) {
    let t115572 = t6897 * t22674 * t31607;
    let t115577 = t1985 * t80707 * t8621;
    let t115583 = t22633 * t22635 * t31549 * t3719;
    let t115586 = t81228 * t81326 * t31550;
    (t115572, t115577, t115583, t115586)
}
