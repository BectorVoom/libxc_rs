//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1274/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1274(t12366: f64, t12367: f64, t12368: f64, t12220: f64, t12223: f64, t11453: f64, t11457: f64, t11460: f64, t11463: f64, t11467: f64, t11471: f64, t41147: f64, t41148: f64, t41149: f64, t41150: f64, t41193: f64, t41237: f64, t41277: f64, t41323: f64, t41809: f64, t42356: f64, t42360: f64, t42364: f64, t8: f64) -> f64 {
    let t42369 = 2.0_f64 * t12366;
    let t42370 = 2.0_f64 * t12367;
    let t42371 = 2.0_f64 * t12368;
    let t42372 = 15.0_f64 / 8.0_f64 * t12220;
    let t42373 = t12223 / 2.0_f64;
    let t42374 = t11471 - t41147 + t41148 + t41149 + t41150 + t8 * (t41193 + t41237 + t41277 + t41323 + t41809 + t42356 + t42360 + t42364) + t42369 + t42370 + t11453 - t11457 - t11460 + t42371 - t42372 - t42373 - t11463 - t11467;
    t42374
}
