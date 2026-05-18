//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1033/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1033<F: Float>(t1161: F, t2352: F, t2409: F, t3067: F, t1105: F, t2376: F, t274: F, t745: F, t820: F, t3258: F, t3257: F, t1123: F, t6686: F) -> (F, F, F, F, F, F, F) {
    let t9321 = t1161 * t2352;
    let t9323 = t2409 * t3067 * t9321;
    let t9326 = t1105 * t2352;
    let t9328 = t2409 * t2376 * t9326;
    let t9332 = t745 * t820 * t274;
    let t9333 = t3258 * t9332;
    let t9334 = t3257 * t9333;
    let t9337 = t1123 * t6686;
    (t9321, t9323, t9326, t9328, t9333, t9334, t9337)
}
