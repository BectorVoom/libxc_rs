//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 694/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk694(t22: f64, t7508: f64, t420: f64, t56: f64, t1072: f64, t368: f64, t7507: f64, t1095: f64, t1083: f64, t355: f64, t360: f64, t7458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7510 = 1.0_f64 / t22 / t7508;
    let t7512 = t7510 * t56 * t420;
    let t7513 = t368 * t1072;
    let t7514 = t7512 * t7513;
    let t7515 = t7507 * t7514;
    let t7517 = t1095 * t1072;
    let t7518 = t7512 * t7517;
    let t7519 = t7507 * t7518;
    let t7528 = t7458 * t1083 * t355 * t360;
    (t7510, t7512, t7513, t7514, t7515, t7517, t7518, t7519, t7528)
}
