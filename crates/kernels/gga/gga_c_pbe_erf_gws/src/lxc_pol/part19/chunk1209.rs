//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1209/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1209<F: Float>(t54321: F, t55591: F, t55593: F, t57151: F, t57154: F, t57156: F, t57158: F, t57160: F, t57162: F, t57164: F, t57166: F, t57168: F, t52696: F, t54331: F, t55596: F, t55603: F, t57171: F, t57174: F, t57176: F, t57179: F, t57182: F, t57184: F, t57186: F, t57188: F, t57191: F) -> (F, F) {
    let t58742 = t57151 / 96.0 - t55591 - t54321 + t57154 / 24.0 - t55593 - t57156 / 24.0 - t57158 / 48.0 + 7.0 / 72.0 * t57160 - t57162 / 48.0 - t57164 / 48.0 - t57166 / 48.0 - t57168 / 384.0;
    let t58752 = -t57171 / 384.0 - t57174 / 48.0 + 7.0 / 576.0 * t57176 + t57179 / 8.0 - t55596 - t54331 - t52696 - 7.0 / 192.0 * t57182 - t57184 / 8.0 - t57186 / 8.0 - 35.0 / 288.0 * t57188 - t57191 / 48.0 - t55603;
    (t58742, t58752)
}
