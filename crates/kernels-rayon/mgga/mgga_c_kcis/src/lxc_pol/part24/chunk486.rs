//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 486/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk486(t287: f64, t3530: f64, t2917: f64, t1207: f64, t1211: f64, t1210: f64, t401: f64, t396: f64, t2966: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3531 = t287 * t3530;
    let t3537 = 0.22831111111111111111e-1_f64 * t2917;
    let t3545 = t1207 * t1211;
    let t3548 = t1210 * t401;
    let t3549 = 1.0_f64 / t3548;
    let t3550 = t396 * t3549;
    let t3557 = 0.68863333333333333333e0_f64 * t2917;
    let t3564 = 0.17365833333333333333e0_f64 * t2966;
    let t3573 = t1210 * t1210;
    let t3574 = 1.0_f64 / t3573;
    let t3575 = t396 * t3574;
    let t3576 = t404 * t404;
    let t3577 = 1.0_f64 / t3576;
    (t3531, t3537, t3545, t3549, t3550, t3557, t3564, t3573, t3574, t3575, t3576, t3577)
}
