//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1345/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1345(t11680: f64, t14015: f64, t11685: f64, t11949: f64, t14007: f64, t51408: f64, t54320: f64, t54323: f64, t57151: f64, t57154: f64, t57156: f64, t57158: f64, t57160: f64, t57162: f64) -> f64 {
    let t57164 = t14015 * t11680;
    let t57166 = t14015 * t11685;
    let t57168 = t14007 * t11949;
    let t57170 = t57151 / 192.0_f64 - t54320 - 35.0_f64 / 432.0_f64 * t51408 + t57154 / 48.0_f64 - t54323 - t57156 / 48.0_f64 - t57158 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t57160 - t57162 / 96.0_f64 - t57164 / 96.0_f64 - t57166 / 96.0_f64 - t57168 / 768.0_f64;
    t57170
}
