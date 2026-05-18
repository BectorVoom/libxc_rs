//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 711/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk711<F: Float>(t2376: F, t2409: F, t4155: F, t1144: F, t1193: F, t338: F, t1161: F, t1192: F) -> (F, F, F) {
    let t4157 = t2409 * t2376 * t4155;
    let t4160 = t1144 * t1193;
    let t4161 = t338 * t4160;
    let t4164 = t1192 * t1161;
    (t4157, t4161, t4164)
}
