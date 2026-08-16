//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1315/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1315(t9397: f64, t195: f64, t40: f64, t197: f64, t52: f64, t138: f64, t2409: f64, t125: f64, t2412: f64, t701: f64, t2414: f64) -> (f64, f64, f64, f64, f64) {
    let t9398 = 1.0_f64 / t9397;
    let t9427 = 1.0_f64 / t195 / t40;
    let t9438 = 1.0_f64 / t197 / t52;
    let t9452 = 1.0_f64 / t2409 / t138;
    let t9453 = t125 * t9452;
    let t9454 = t2412 * t701;
    let t9455 = t9454 * t2414;
    let t9457 = 0.96491876992155210402e2_f64 * t9453 * t9455;
    (t9398, t9427, t9438, t9454, t9457)
}
