//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1393/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1393(t27921: f64, t6534: f64, t24972: f64, t26542: f64, t26545: f64, t105108: f64, t7769: f64, t120792: f64, t120793: f64, t120795: f64, t120800: f64, t120803: f64, t120804: f64, t120807: f64) -> f64 {
    let t123282 = t27921 * t6534;
    let t123285 = t24972 * t26542;
    let t123287 = t24972 * t26545;
    let t123290 = t105108 * t7769;
    let t123292 = t120792 + 0.135e2_f64 * t120793 + 0.135e2_f64 * t123282 + 27.0_f64 * t120795 + t120800 + t120803 + 27.0_f64 * t123285 + 27.0_f64 * t123287 + 27.0_f64 * t120804 + t120807 + 27.0_f64 * t123290;
    t123292
}
