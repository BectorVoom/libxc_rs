//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 915/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk915(t8480: f64, t967: f64, t140: f64, t2699: f64, t925: f64, t2464: f64, t265: f64, t2458: f64, t606: f64, t2645: f64, t2719: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8481 = t967 * t8480;
    let t8483 = t140 * t2699;
    let t8484 = t925 * t8483;
    let t8491 = 1.0_f64 / t265 / t2464;
    let t8493 = 1.0_f64 / t2458 / t606;
    let t8499 = t140 * t2645;
    let t8500 = t925 * t8499;
    let t8507 = t2719 * t72;
    (t8481, t8484, t8491, t8493, t8500, t8507)
}
