//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1359/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1359<F: Float>(t1192: F, t13911: F, t14420: F, t15139: F, t22343: F, t2376: F, t2408: F, t2409: F, t29775: F, t3207: F, t3703: F, t39689: F, t4052: F, t53075: F, t53943: F, t53948: F, t53953: F, t53959: F, t57358: F, t57361: F, t57371: F, t57373: F, t57375: F, t57379: F, t8793: F, t9807: F) -> F {
    let t57381 = -t3207 * t2409 * t2376 * t4052 * t3703 / F::cast_from(16.0_f64) + t2408 * t2409 * t2376 * t1192 * t9807 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57358 + t57361 / F::cast_from(768.0_f64) + t39689 * t13911 / F::cast_from(48.0_f64) - t53943 + t29775 * t14420 / F::cast_from(24.0_f64) + t22343 * t15139 / F::cast_from(96.0_f64) + t53948 + t8793 * t53075 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t57371 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57373 + t57375 / F::cast_from(48.0_f64) + t53953 + F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t53959 + t57379 / F::cast_from(16.0_f64);
    t57381
}
