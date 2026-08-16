//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 733/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk733(t353: f64, t4228: f64, t338: f64, t1115: f64, t2408: f64, t3066: f64, t335: f64, t4072: f64, t4077: f64, t4083: f64, t4087: f64, t4128: f64, t4131: f64, t4133: f64, t4136: f64, t4139: f64, t4143: f64, t4147: f64, t4151: f64, t4209: f64, t4213: f64, t4218: f64) -> (f64, f64) {
    let t4229 = t353 * t4228;
    let t4230 = t338 * t4229;
    let t4233 = t4128 / 48.0_f64 - t4072 - t4131 / 24.0_f64 + t4133 / 48.0_f64 - t4136 / 48.0_f64 + t4139 / 768.0_f64 - t4077 - t4143 / 384.0_f64 - t4147 / 1536.0_f64 - t4151 / 1536.0_f64 - t1115 * t4083 / 96.0_f64 + t4087 + t2408 * t4209 / 48.0_f64 - t335 * t4213 / 96.0_f64 + t3066 * t4218 / 48.0_f64 - t335 * t4230 / 96.0_f64;
    (t4230, t4233)
}
