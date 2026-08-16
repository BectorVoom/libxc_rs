//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 731/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk731(t2354: f64, t33341: f64, t684: f64, t6118: f64, t713: f64, t7484: f64, t2506: f64, t1434: f64, t193: f64, t202: f64, t7446: f64, t237: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33343 = t2354 * t33341 * t684;
    let t33344 = t6118 * t33343;
    let t33346 = t7484 * t713;
    let t33347 = t2506 * t33346;
    let t33349 = t1434 * t193 * t33347;
    let t33350 = t202 * t7446;
    let t33351 = t33350 * t237;
    (t33343, t33344, t33346, t33347, t33349, t33350, t33351)
}
