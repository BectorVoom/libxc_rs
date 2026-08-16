//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1142/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1142(t214: f64, t6434: f64, t22751: f64, t28213: f64, t28210: f64, t28233: f64, t6883: f64, t22674: f64, t28232: f64, t6897: f64, t28195: f64, t28199: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97511 = t214 * t6434;
    let t97529 = t22751 * t28213;
    let t97537 = t22751 * t28210;
    let t97548 = t6883 * t28233;
    let t97571 = t6897 * t22674 * t28232;
    let t97573 = t6883 * t28195;
    let t97599 = t6897 * t794 * t28199;
    (t97511, t97529, t97537, t97548, t97571, t97573, t97599)
}
