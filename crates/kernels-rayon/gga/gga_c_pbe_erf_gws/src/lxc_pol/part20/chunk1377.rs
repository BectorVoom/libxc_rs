//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1377/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1377(t1115: f64, t11342: f64, t13772: f64, t3921: f64, t4002: f64, t51967: f64, t54617: f64, t54711: f64, t55892: f64, t57626: f64, t57635: f64, t57639: f64, t57641: f64, t57643: f64, t57648: f64, t57650: f64, t57652: f64, t57654: f64) -> f64 {
    let t57656 = -t1115 * t54711 / 48.0_f64 - t57626 / 768.0_f64 - t11342 * t4002 / 96.0_f64 - t3921 * t13772 / 96.0_f64 - t57635 / 1536.0_f64 + t54617 - 35.0_f64 / 432.0_f64 * t51967 - t55892 - t57639 / 96.0_f64 + 7.0_f64 / 1152.0_f64 * t57641 + 7.0_f64 / 288.0_f64 * t57643 - t57648 / 768.0_f64 + 7.0_f64 / 288.0_f64 * t57650 + 7.0_f64 / 2304.0_f64 * t57652 + t57654 / 24.0_f64;
    t57656
}
