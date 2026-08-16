//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1269/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1269(t28182: f64, t6914: f64, t22893: f64, t28142: f64, t80681: f64, t28143: f64, t80727: f64, t1338: f64, t28107: f64, t28160: f64, t6883: f64, t6396: f64, t80820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97148 = t6914 * t28182;
    let t97161 = t80681 * t22893 * t28142;
    let t97179 = t80727 * t28143;
    let t97193 = t1338 * t28107;
    let t97200 = t6883 * t28160;
    let t97219 = t80820 * t6396;
    (t97148, t97161, t97179, t97193, t97200, t97219)
}
