//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1328/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1328(t54377: f64, t4171: f64, t51407: f64, t4049: f64, t9661: f64, t4043: f64, t9449: f64, t51437: f64, t51439: f64, t51447: f64, t51452: f64, t54366: f64, t54368: f64, t54370: f64, t54374: f64) -> f64 {
    let t54378 = 7.0_f64 / 72.0_f64 * t54377;
    let t54381 = t51407 * t4171;
    let t54384 = t4049 * t9661;
    let t54386 = t4043 * t9449;
    let t54388 = -t54366 / 384.0_f64 - t54368 / 96.0_f64 - t54370 / 96.0_f64 + t54374 / 48.0_f64 + 7.0_f64 / 288.0_f64 * t51437 + t54378 + 7.0_f64 / 144.0_f64 * t51439 + 7.0_f64 / 576.0_f64 * t51447 - 35.0_f64 / 432.0_f64 * t54381 + 7.0_f64 / 1152.0_f64 * t51452 - t54384 / 384.0_f64 - t54386 / 768.0_f64;
    t54388
}
