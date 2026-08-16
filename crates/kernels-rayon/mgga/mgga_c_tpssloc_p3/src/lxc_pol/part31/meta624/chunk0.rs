//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1881/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1881(t22893: f64, t28142: f64, t80681: f64, t28143: f64, t80727: f64, t28160: f64, t6883: f64, t19873: f64, t26309: f64, t19966: f64, t6396: f64, t80816: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97161 = t80681 * t22893 * t28142;
    let t97179 = t80727 * t28143;
    let t97200 = t6883 * t28160;
    let t97202 = t26309 * t19873;
    let t97204 = t26309 * t19966;
    let t97206 = t80816 * t6396;
    (t97161, t97179, t97200, t97202, t97204, t97206)
}
