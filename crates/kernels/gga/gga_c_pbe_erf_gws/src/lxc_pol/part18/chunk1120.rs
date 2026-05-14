//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1120/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1120<F: Float>(t11396: F, t13888: F, t2408: F, t52931: F, t53083: F, t53779: F, t56110: F, t56113: F, t56116: F, t56119: F, t56124: F, t56126: F, t56129: F, t56133: F, t56142: F, t56147: F, t56153: F, t6793: F, t8793: F, t9283: F) -> (F,) {
    let t56155 = -t56110 / 48.0 - t56113 / 48.0 + t56116 / 48.0 + t56119 / 16.0 + t56124 / 96.0 - 7.0 / 1152.0 * t56126 - t56129 / 768.0 - t6793 * t56133 / 12.0 - t8793 * t53083 / 12.0 - t8793 * t53779 / 12.0 - 7.0 / 288.0 * t56142 - 7.0 / 288.0 * t56147 - t52931 - t2408 * t9283 * t13888 * t11396 / 24.0 + t56153 / 48.0;
    (t56155,)
}
