//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 630/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk630(t123: f64, t131: f64, t2387: f64, t2390: f64, t693: f64, t119: f64, t63: f64, t133: f64) -> (f64, f64, f64, f64) {
    let t2396 = 1.0_f64/f64::sqrt(t123);
    let t2397 = t2396 * t131;
    let t2398 = t2397 * t2387;
    let t2400 = t693 * t2390;
    let t2402 = t119 * t63;
    let t2403 = t133 * t2402;
    (t2397, t2398, t2400, t2403)
}
