//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 793/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk793(t2960: f64, t2971: f64, t2970: f64, t2995: f64, t973: f64, t2769: f64, t40: f64, t344: f64, t9288: f64, t2979: f64, t338: f64, t9277: f64) -> (f64, f64, f64, f64, f64) {
    let t10267 = t2960 * t2971;
    let t10273 = t2970 * t2995;
    let t10274 = t973 * t10273;
    let t10276 = t2769 * t40;
    let t10277 = 1.0_f64 / t10276;
    let t10278 = t344 * t10277;
    let t10279 = t10278 * t9288;
    let t10280 = t2979 * t10279;
    let t10283 = t9277 * t338;
    (t10267, t10274, t10277, t10280, t10283)
}
