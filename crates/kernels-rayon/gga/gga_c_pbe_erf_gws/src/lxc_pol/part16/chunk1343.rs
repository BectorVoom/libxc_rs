//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1343/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1343(t54344: f64, t54352: f64, t54354: f64, t54356: f64, t51431: f64, t54338: f64, t54342: f64, t54346: f64, t54348: f64, t54350: f64, t54360: f64, t54362: f64) -> f64 {
    let t55603 = 35.0_f64 / 144.0_f64 * t54344;
    let t55607 = 119.0_f64 / 864.0_f64 * t54352;
    let t55608 = 7.0_f64 / 144.0_f64 * t54354;
    let t55609 = 35.0_f64 / 108.0_f64 * t54356;
    let t55613 = -5.0_f64 / 48.0_f64 * t54338 + t54342 / 24.0_f64 - t55603 - 5.0_f64 / 32.0_f64 * t54346 - t54348 / 24.0_f64 - t54350 / 48.0_f64 - t55607 + t55608 - t55609 + 7.0_f64 / 72.0_f64 * t51431 + t54360 / 4.0_f64 + t54362 / 192.0_f64;
    t55613
}
