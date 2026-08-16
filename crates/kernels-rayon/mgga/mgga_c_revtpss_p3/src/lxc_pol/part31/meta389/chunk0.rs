//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1427/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1427(t1647: f64, t3298: f64, t1086: f64, t1678: f64, t994: f64, t12166: f64, t378: f64, t342: f64, t11631: f64, t12050: f64, t12077: f64, t3154: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16509 = t1647 * t3298;
    let t16543 = t1086 * t1678;
    let t16544 = t994 * t16543;
    let t16551 = t12166 * t378;
    let t16552 = t342 * t16551;
    let t16553 = t12050 * t11631;
    let t16558 = t12077 * t378;
    let t16559 = t342 * t16558;
    let t16560 = t12050 * t3154;
    (t16509, t16544, t16552, t16553, t16559, t16560)
}
