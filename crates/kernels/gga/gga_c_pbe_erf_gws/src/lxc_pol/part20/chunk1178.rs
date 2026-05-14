//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1178/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1178<F: Float>(t14058: F, t3875: F, t36666: F, t850: F, t14093: F, t51412: F, t51415: F, t54330: F, t54345: F, t57171: F, t57174: F, t57176: F, t57179: F, t57182: F, t57184: F, t57186: F) -> (F,) {
    let t57188 = t14058 * t3875;
    let t57190 = t850 * t36666;
    let t57191 = t57190 * t14093;
    let t57193 = -t57171 / 768.0 - t57174 / 96.0 + 7.0 / 1152.0 * t57176 + t57179 / 16.0 - t54330 - 35.0 / 216.0 * t51412 - t51415 - 7.0 / 384.0 * t57182 - t57184 / 16.0 - t57186 / 16.0 - 35.0 / 576.0 * t57188 - t57191 / 96.0 - t54345;
    (t57193,)
}
