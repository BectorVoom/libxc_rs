//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 813/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk813(t10277: f64, t344: f64, t241: f64, t625: f64, t281: f64, t283: f64, t2978: f64, t340: f64, t63: f64, t221: f64, t339: f64, t2393: f64, t374: f64, t376: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10278 = t344 * t10277;
    let t10292 = t625 * t241;
    let t10294 = t281 * t10292 * t283;
    let t10295 = 20.0_f64 / 27.0_f64 * t10294;
    let t10304 = t241 * t2978;
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = 0.3086419753086419753e-3_f64 * t339 * t10337;
    let t10375 = t374 * t2393 * t376;
    (t10278, t10292, t10294, t10295, t10304, t10335, t10339, t10375)
}
