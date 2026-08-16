//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1136/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1136(t28182: f64, t6914: f64, t22893: f64, t28142: f64, t80681: f64, t28143: f64, t80727: f64, t28160: f64, t6883: f64, t6396: f64, t80820: f64, t28101: f64, t80958: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97148 = t6914 * t28182;
    let t97161 = t80681 * t22893 * t28142;
    let t97179 = t80727 * t28143;
    let t97200 = t6883 * t28160;
    let t97219 = t80820 * t6396;
    let t97238 = t80958 * t28101;
    (t97148, t97161, t97179, t97200, t97219, t97238)
}
