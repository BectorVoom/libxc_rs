//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1359/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1359<F: Float>(t1192: F, t13911: F, t14420: F, t15139: F, t22343: F, t2376: F, t2408: F, t2409: F, t29775: F, t3207: F, t3703: F, t39689: F, t4052: F, t53075: F, t53943: F, t53948: F, t53953: F, t53959: F, t57358: F, t57361: F, t57371: F, t57373: F, t57375: F, t57379: F, t8793: F, t9807: F) -> F {
    let t57381 = -t3207 * t2409 * t2376 * t4052 * t3703 / F::new(16.0) + t2408 * t2409 * t2376 * t1192 * t9807 / F::new(48.0) - F::new(7.0) / F::new(144.0) * t57358 + t57361 / F::new(768.0) + t39689 * t13911 / F::new(48.0) - t53943 + t29775 * t14420 / F::new(24.0) + t22343 * t15139 / F::new(96.0) + t53948 + t8793 * t53075 / F::new(24.0) - F::new(7.0) / F::new(2304.0) * t57371 + F::new(7.0) / F::new(288.0) * t57373 + t57375 / F::new(48.0) + t53953 + F::new(35.0) / F::new(108.0) * t53959 + t57379 / F::new(16.0);
    t57381
}
