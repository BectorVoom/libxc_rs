//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2174/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2174(t28116: f64, t81228: f64, t81326: f64, t6897: f64, t7700: f64, t90544: f64, t214: f64, t6434: f64, t1985: f64, t6907: f64, t22633: f64, t26215: f64, t90566: f64) -> (f64, f64, f64, f64, f64) {
    let t97503 = t81228 * t81326 * t28116;
    let t97509 = t6897 * t90544 * t7700;
    let t97511 = t214 * t6434;
    let t97513 = t1985 * t97511 * t6907;
    let t97516 = t22633 * t90566 * t26215;
    (t97503, t97509, t97511, t97513, t97516)
}
