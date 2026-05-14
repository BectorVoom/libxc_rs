//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1094/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1094<F: Float>(t14617: F, t50943: F, t3989: F, t3990: F, t3991: F, t9080: F, t345: F, t6126: F, t9297: F, t1161: F, t14106: F, t2409: F, t3066: F, t3067: F, t53234: F, t53238: F, t53243: F, t53246: F, t53248: F, t53251: F, t53253: F, t53261: F, t53264: F, t53266: F, t53270: F, t9283: F) -> (F,) {
    let t53272 = t50943 * t14617;
    let t53273 = 7.0 / 144.0 * t53272;
    let t53276 = t3989 * t3990 * t3991 * t9080;
    let t53283 = t345 * t6126;
    let t53286 = t3989 * t3990 * t53283 * t9297;
    let t53288 = -t53234 / 48.0 + t53238 / 384.0 - t53243 / 768.0 - t53246 / 24.0 - t53248 / 96.0 - t53251 / 48.0 + t3066 * t9283 * t53253 * t9297 / 4.0 - t53261 + t53264 / 3072.0 + t53266 / 48.0 + t53270 / 512.0 + t53273 - t53276 / 3072.0 + t3066 * t2409 * t3067 * t14106 * t1161 / 48.0 - t53286 / 512.0;
    (t53288,)
}
