//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1186/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1186<F: Float>(t13953: F, t15164: F, t11547: F, t13917: F, t53156: F, t15296: F, t3979: F, t15300: F, t840: F, t11356: F, t3965: F, t14121: F, t2409: F, t39579: F, t1192: F, t13911: F, t14420: F, t15139: F, t22343: F, t2376: F, t2408: F, t29775: F, t3207: F, t3703: F, t39689: F, t4052: F, t53075: F, t53943: F, t53948: F, t53953: F, t53959: F, t8793: F, t9807: F) -> (F,) {
    let t57358 = t13953 * t15164;
    let t57361 = t13917 * t53156 * t11547;
    let t57371 = t3979 * t15296;
    let t57373 = t840 * t15300;
    let t57375 = t3965 * t11356;
    let t57379 = t14121 * t2409 * t39579;
    let t57381 = -t3207 * t2409 * t2376 * t4052 * t3703 / 16.0 + t2408 * t2409 * t2376 * t1192 * t9807 / 48.0 - 7.0 / 144.0 * t57358 + t57361 / 768.0 + t39689 * t13911 / 48.0 - t53943 + t29775 * t14420 / 24.0 + t22343 * t15139 / 96.0 + t53948 + t8793 * t53075 / 24.0 - 7.0 / 2304.0 * t57371 + 7.0 / 288.0 * t57373 + t57375 / 48.0 + t53953 + 35.0 / 108.0 * t53959 + t57379 / 16.0;
    (t57381,)
}
