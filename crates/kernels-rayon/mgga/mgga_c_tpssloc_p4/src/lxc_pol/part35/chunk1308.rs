//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1308/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1308(t28160: f64, t6883: f64, t6396: f64, t80820: f64, t28101: f64, t80958: f64, t1827: f64, t91285: f64, t19815: f64, t6944: f64, t22765: f64, t6422: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97200 = t6883 * t28160;
    let t97219 = t80820 * t6396;
    let t97238 = t80958 * t28101;
    let t97240 = t91285 * t1827;
    let t97246 = t19815 * t6944;
    let t97253 = t22765 * t6422;
    (t97200, t97219, t97238, t97240, t97246, t97253)
}
