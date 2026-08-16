//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1344/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1344(t54377: f64, t54381: f64, t51437: f64, t51439: f64, t51447: f64, t51452: f64, t54366: f64, t54368: f64, t54370: f64, t54374: f64, t54384: f64, t54386: f64) -> f64 {
    let t55620 = 7.0_f64 / 36.0_f64 * t54377;
    let t55623 = 35.0_f64 / 216.0_f64 * t54381;
    let t55627 = -t54366 / 192.0_f64 - t54368 / 48.0_f64 - t54370 / 48.0_f64 + t54374 / 24.0_f64 + 7.0_f64 / 144.0_f64 * t51437 + t55620 + 7.0_f64 / 72.0_f64 * t51439 + 7.0_f64 / 288.0_f64 * t51447 - t55623 + 7.0_f64 / 576.0_f64 * t51452 - t54384 / 192.0_f64 - t54386 / 384.0_f64;
    t55627
}
