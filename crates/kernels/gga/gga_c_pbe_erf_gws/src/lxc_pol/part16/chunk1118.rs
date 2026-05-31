//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1118/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1118<F: Float>(t4094: F, t840: F, t1206: F, t2220: F, t338: F, t1205: F, t2352: F, t2409: F, t3067: F, t13894: F, t13833: F, t13856: F, t13862: F, t13866: F, t13870: F, t13873: F, t13878: F, t13896: F, t13900: F, t14260: F, t14266: F, t14274: F, t14280: F, t2408: F, t3066: F, t335: F) -> (F, F, F, F, F, F) {
    let t14283 = t840 * t4094;
    let t14286 = t338 * t2220 * t1206;
    let t14289 = t1205 * t2352;
    let t14291 = t2409 * t3067 * t14289;
    let t14295 = F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t13894;
    let t14298 = t2408 * t14260 / F::cast_from(24.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t13833 - t3066 * t14266 / F::cast_from(16.0_f64) - t13856 / F::cast_from(24.0_f64) + t13862 / F::cast_from(192.0_f64) + t13866 / F::cast_from(192.0_f64) + t3066 * t14274 / F::cast_from(24.0_f64) - t13870 / F::cast_from(1536.0_f64) + t13873 / F::cast_from(24.0_f64) - t335 * t14280 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14283 - t335 * t14286 / F::cast_from(96.0_f64) + t3066 * t14291 / F::cast_from(48.0_f64) + t13878 / F::cast_from(384.0_f64) + t14295 + t13896 / F::cast_from(24.0_f64) - t13900 / F::cast_from(1536.0_f64);
    (t14283, t14286, t14289, t14291, t14295, t14298)
}
