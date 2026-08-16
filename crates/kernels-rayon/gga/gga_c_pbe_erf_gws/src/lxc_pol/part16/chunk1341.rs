//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1341/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1341(t54301: f64, t54305: f64, t51383: f64, t51388: f64, t51396: f64, t51401: f64, t54295: f64, t54297: f64, t54299: f64, t54303: f64, t54307: f64, t54310: f64) -> f64 {
    let t55580 = 7.0_f64 / 288.0_f64 * t54301;
    let t55582 = 119.0_f64 / 1728.0_f64 * t54305;
    let t55586 = -7.0_f64 / 72.0_f64 * t51383 - 119.0_f64 / 864.0_f64 * t51388 - 119.0_f64 / 432.0_f64 * t51396 + t54295 / 24.0_f64 - t54297 / 12.0_f64 + t54299 / 24.0_f64 + t55580 + 5.0_f64 / 96.0_f64 * t54303 - t55582 - t54307 / 24.0_f64 - 35.0_f64 / 288.0_f64 * t51401 + t54310 / 96.0_f64;
    t55586
}
