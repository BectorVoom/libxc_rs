//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1314/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1314(t53260: f64, t53272: f64, t1205: f64, t12213: f64, t14289: f64, t2376: f64, t2408: f64, t2409: f64, t3066: f64, t52217: f64, t53243: f64, t53246: f64, t53248: f64, t53251: f64, t53264: f64, t53266: f64, t53270: f64, t53276: f64, t53286: f64, t53299: f64, t8574: f64) -> f64 {
    let t55031 = 7.0_f64 / 72.0_f64 * t53260;
    let t55036 = 7.0_f64 / 72.0_f64 * t53272;
    let t55049 = -t53243 / 384.0_f64 - t53246 / 12.0_f64 - t53248 / 48.0_f64 - t53251 / 24.0_f64 - t55031 + 7.0_f64 / 36.0_f64 * t52217 + t53264 / 1536.0_f64 + t53266 / 24.0_f64 + t53270 / 256.0_f64 + t55036 - t53276 / 1536.0_f64 - t53286 / 256.0_f64 + t3066 * t2409 * t12213 * t14289 / 48.0_f64 + t2408 * t2409 * t2376 * t1205 * t8574 / 48.0_f64 + t53299 / 384.0_f64;
    t55049
}
