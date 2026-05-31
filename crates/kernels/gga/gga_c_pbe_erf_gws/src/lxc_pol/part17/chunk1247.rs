//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1247/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1247<F: Float>(t53272: F, t3989: F, t3990: F, t3991: F, t9080: F, t345: F, t6126: F, t9297: F, t1161: F, t14106: F, t2409: F, t3066: F, t3067: F, t53234: F, t53238: F, t53243: F, t53246: F, t53248: F, t53251: F, t53253: F, t53261: F, t53264: F, t53266: F, t53270: F, t9283: F) -> F {
    let t53273 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t53272;
    let t53276 = t3989 * t3990 * t3991 * t9080;
    let t53283 = t345 * t6126;
    let t53286 = t3989 * t3990 * t53283 * t9297;
    let t53288 = -t53234 / F::cast_from(48.0_f64) + t53238 / F::cast_from(384.0_f64) - t53243 / F::cast_from(768.0_f64) - t53246 / F::cast_from(24.0_f64) - t53248 / F::cast_from(96.0_f64) - t53251 / F::cast_from(48.0_f64) + t3066 * t9283 * t53253 * t9297 / F::cast_from(4.0_f64) - t53261 + t53264 / F::cast_from(3072.0_f64) + t53266 / F::cast_from(48.0_f64) + t53270 / F::cast_from(512.0_f64) + t53273 - t53276 / F::cast_from(3072.0_f64) + t3066 * t2409 * t3067 * t14106 * t1161 / F::cast_from(48.0_f64) - t53286 / F::cast_from(512.0_f64);
    t53288
}
