//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1382/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1382(t11363: f64, t11407: f64, t1193: f64, t14791: f64, t15337: f64, t2408: f64, t29751: f64, t3066: f64, t3742: f64, t51084: f64, t54667: f64, t54682: f64, t57694: f64, t57696: f64, t57700: f64, t57702: f64, t57705: f64, t57707: f64, t57711: f64, t57719: f64, t57731: f64, t9241: f64, t9283: f64) -> f64 {
    let t57737 = t54667 - t57694 / 24.0_f64 + 7.0_f64 / 72.0_f64 * t57696 - t57700 / 768.0_f64 + 7.0_f64 / 144.0_f64 * t57702 - t57705 / 24.0_f64 - t54682 - 7.0_f64 / 288.0_f64 * t57707 + t57711 / 768.0_f64 + t9241 * t9283 * t1193 * t11363 / 4.0_f64 + t57719 / 384.0_f64 - t3066 * t9283 * t14791 * t11407 / 8.0_f64 - t2408 * t29751 * t15337 / 12.0_f64 + t57731 / 3072.0_f64 - t2408 * t9283 * t51084 * t3742 / 12.0_f64;
    t57737
}
