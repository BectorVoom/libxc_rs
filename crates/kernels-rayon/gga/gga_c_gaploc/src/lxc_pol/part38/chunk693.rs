//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 693/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk693(t3553: f64, t921: f64, t4349: f64, t1016: f64, t3418: f64, t1382: f64, t2355: f64, t3599: f64, t11402: f64, t895: f64, t11386: f64, t2778: f64, t3338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13343 = t3553 * t921;
    let t13345 = 6.0_f64 * t4349 * t13343;
    let t13346 = t1016 * t3418;
    let t13348 = 4.0_f64 * t1382 * t13346;
    let t13349 = t2355 * t3599;
    let t13350 = t3599 * t921;
    let t13352 = 2.0_f64 * t1382 * t13350;
    let t13354 = 0.35750489951850426669e0_f64 * t895 * t11402;
    let t13356 = 0.35750489951850426669e0_f64 * t895 * t11386;
    let t13359 = t2778 * t3338;
    (t13343, t13345, t13346, t13348, t13349, t13350, t13352, t13354, t13356, t13359)
}
