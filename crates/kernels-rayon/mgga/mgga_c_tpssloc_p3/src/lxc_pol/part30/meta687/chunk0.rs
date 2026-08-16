//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2177/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2177(t225: f64, t28108: f64, t22674: f64, t28232: f64, t6897: f64, t28195: f64, t6883: f64, t22633: f64, t22635: f64, t26337: f64, t5353: f64, t5325: f64, t90488: f64) -> (f64, f64, f64, f64, f64) {
    let t97558 = t28108 * t225;
    let t97571 = t6897 * t22674 * t28232;
    let t97573 = t6883 * t28195;
    let t97577 = t22633 * t22635 * t26337 * t5353;
    let t97583 = t22633 * t22635 * t90488 * t5325;
    (t97558, t97571, t97573, t97577, t97583)
}
