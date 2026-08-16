//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1115/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1115(t2511: f64, t39377: f64, t39378: f64, t1294: f64, t1307: f64, t3918: f64, t39335: f64, t39338: f64, t39340: f64, t39342: f64, t39346: f64, t39349: f64, t39350: f64, t39356: f64, t39360: f64, t39364: f64, t39366: f64, t39367: f64, t39373: f64, t39375: f64, t6999: f64) -> (f64, f64, f64, f64) {
    let t39380 = t2511 * t2511;
    let t39381 = 1.0_f64 / t39380;
    let t39382 = t39377 * t39378 * t39381;
    let t39384 = 0.91082604192152556044e5_f64 * t1294 * t39382;
    let t39385 = 24.0_f64 * t1307 * t3918 * t39350 - 36.0_f64 * t3918 * t39367 * t6999 - t39335 - t39338 + t39340 - t39342 + t39346 + t39349 + t39356 + t39360 + t39364 - t39366 + t39373 - t39375 - t39384;
    (t39381, t39382, t39384, t39385)
}
