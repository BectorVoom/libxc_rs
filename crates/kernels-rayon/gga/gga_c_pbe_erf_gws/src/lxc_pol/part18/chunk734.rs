//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 734/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk734(t353: f64, t4183: f64, t338: f64, t1115: f64, t2408: f64, t3066: f64, t335: f64, t3957: f64, t3981: f64, t4002: f64, t4006: f64, t4128: f64, t4131: f64, t4133: f64, t4136: f64, t4139: f64, t4143: f64, t4147: f64, t4151: f64, t4157: f64, t4161: f64, t4166: f64) -> (f64, f64) {
    let t4184 = t353 * t4183;
    let t4185 = t338 * t4184;
    let t4188 = t4128 / 96.0_f64 - t3957 - t4131 / 48.0_f64 + t4133 / 96.0_f64 - t4136 / 96.0_f64 + t4139 / 1536.0_f64 - t3981 - t4143 / 768.0_f64 - t4147 / 3072.0_f64 - t4151 / 3072.0_f64 - t1115 * t4002 / 96.0_f64 + t4006 + t2408 * t4157 / 48.0_f64 - t335 * t4161 / 96.0_f64 + t3066 * t4166 / 48.0_f64 - t335 * t4185 / 96.0_f64;
    (t4185, t4188)
}
