//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 376/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk376<F: Float>(t1184: F, t1185: F, t328: F, t356: F, t361: F, t372: F) -> (F, F) {
    let t1186 = t1184 * t1185;
    let t1189 = t356 * t361 * t328;
    let t1190 = t1189 * t372;
    let t1192 = t1186 / 96.0 + t1190 / 1536.0;
    (t1189, t1192)
}
