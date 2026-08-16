//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 628/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk628(t123: f64, t126: f64, t131: f64, t119: f64, t132: f64, t63: f64, t204: f64, t686: f64, t685: f64, t120: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2385 = 1.0_f64 / t126 / t123 * t131;
    let t2386 = t132 * t119;
    let t2387 = t2386 * t63;
    let t2388 = t2385 * t2387;
    let t2390 = t686 * t204;
    let t2391 = t685 * t2390;
    let t2393 = t120 * t204;
    (t2385, t2386, t2387, t2388, t2390, t2391, t2393)
}
