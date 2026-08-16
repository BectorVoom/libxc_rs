//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1078/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1078(t20173: f64, t8319: f64, t1873: f64, t6534: f64, t3941: f64, t3938: f64, t8326: f64, t671: f64, t31253: f64, t31267: f64, t31270: f64, t31272: f64, t31274: f64, t31277: f64, t577: f64, t8508: f64) -> (f64, f64, f64, f64, f64) {
    let t31279 = 27.0_f64 * t20173 * t8319;
    let t31280 = t1873 * t6534;
    let t31282 = 54.0_f64 * t3941 * t31280;
    let t31283 = t3938 * t8326;
    let t31284 = 0.135e2_f64 * t31283;
    let t31285 = t8326 * t671;
    let t31286 = t3941 * t31285;
    let t31287 = 27.0_f64 * t31286;
    let t31288 = 0.45e1_f64 * t31253 * t577 + 0.135e2_f64 * t31267 * t671 + 27.0_f64 * t31270 + 54.0_f64 * t31272 + 27.0_f64 * t31274 + t31277 + t31279 + t31282 + t31284 + t31287 + t8508;
    (t31280, t31284, t31285, t31287, t31288)
}
