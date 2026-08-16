//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1188/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1188<F: Float>(t13895: F, t14931: F, t14962: F, t14974: F, t15187: F, t15192: F, t15195: F, t15198: F, t15201: F, t15205: F, t15209: F, t15213: F, t15216: F, t15275: F, t15279: F, t15283: F, t2408: F, t3066: F, t3207: F, t335: F, t3913: F, t4002: F) -> F {
    let t15285 = t15187 / F::cast_from(1536.0_f64) - t3913 * t4002 / F::cast_from(96.0_f64) - t15192 / F::cast_from(192.0_f64) + t3066 * t15195 / F::cast_from(24.0_f64) + t14931 + t15198 / F::cast_from(24.0_f64) + t15201 / F::cast_from(768.0_f64) - t15205 / F::cast_from(768.0_f64) - t3207 * t15209 / F::cast_from(16.0_f64) + t2408 * t15213 / F::cast_from(24.0_f64) + t15216 / F::cast_from(48.0_f64) + t13895 - t335 * t15275 / F::cast_from(96.0_f64) + t14962 + t15279 / F::cast_from(1536.0_f64) + t15283 / F::cast_from(384.0_f64) - t14974;
    t15285
}
