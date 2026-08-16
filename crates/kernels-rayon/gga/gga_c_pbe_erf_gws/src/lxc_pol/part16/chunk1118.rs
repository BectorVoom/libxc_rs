//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1118/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1118(t4094: f64, t840: f64, t1206: f64, t2220: f64, t338: f64, t1205: f64, t2352: f64, t2409: f64, t3067: f64, t13894: f64, t13833: f64, t13856: f64, t13862: f64, t13866: f64, t13870: f64, t13873: f64, t13878: f64, t13896: f64, t13900: f64, t14260: f64, t14266: f64, t14274: f64, t14280: f64, t2408: f64, t3066: f64, t335: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14283 = t840 * t4094;
    let t14286 = t338 * t2220 * t1206;
    let t14289 = t1205 * t2352;
    let t14291 = t2409 * t3067 * t14289;
    let t14295 = 119.0_f64 / 6912.0_f64 * t13894;
    let t14298 = t2408 * t14260 / 24.0_f64 + 5.0_f64 / 384.0_f64 * t13833 - t3066 * t14266 / 16.0_f64 - t13856 / 24.0_f64 + t13862 / 192.0_f64 + t13866 / 192.0_f64 + t3066 * t14274 / 24.0_f64 - t13870 / 1536.0_f64 + t13873 / 24.0_f64 - t335 * t14280 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t14283 - t335 * t14286 / 96.0_f64 + t3066 * t14291 / 48.0_f64 + t13878 / 384.0_f64 + t14295 + t13896 / 24.0_f64 - t13900 / 1536.0_f64;
    (t14283, t14286, t14289, t14291, t14295, t14298)
}
