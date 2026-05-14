//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1153/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1153<F: Float>(t20091: F, t4209: F, t53577: F, t53583: F, t53597: F, t4110: F, t6126: F, t14185: F, t3066: F, t3068: F, t3207: F, t52331: F, t53553: F, t53562: F, t53564: F, t53567: F, t53572: F, t53579: F, t53581: F, t53595: F, t9213: F, t9283: F) -> (F,) {
    let t55243 = t20091 * t4209;
    let t55248 = 7.0 / 72.0 * t53577;
    let t55251 = 7.0 / 576.0 * t53583;
    let t55258 = 7.0 / 288.0 * t53597;
    let t55259 = t6126 * t4110;
    let t55264 = t53553 / 384.0 - t53562 / 384.0 + 35.0 / 216.0 * t55243 - t53564 / 24.0 + t53567 / 24.0 - t53572 / 12.0 - t55248 - t53579 / 24.0 - t53581 / 24.0 - t55251 - 7.0 / 72.0 * t52331 - 5.0 / 64.0 * t53595 + t3207 * t9283 * t14185 * t9213 / 8.0 - t55258 - t3066 * t9283 * t55259 * t3068 / 8.0;
    (t55264,)
}
