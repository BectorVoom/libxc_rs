//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1394/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1394(t1229: f64, t676: f64, t1090: f64, t248: f64, t1227: f64, t3536: f64, t3572: f64, t3252: f64, t3521: f64, t3248: f64, t1009: f64, t3481: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11789 = t676 * t1229;
    let t11791 = t248 * t11789 * t1090;
    let t11792 = t1227 * t11791;
    let t11794 = t3536 * t3572;
    let t11797 = t248 * t3521 * t3252;
    let t11798 = t1227 * t11797;
    let t11801 = t248 * t3521 * t3248;
    let t11802 = t1227 * t11801;
    let t11812 = t3481 * t1009;
    (t11789, t11791, t11792, t11794, t11797, t11798, t11801, t11802, t11812)
}
