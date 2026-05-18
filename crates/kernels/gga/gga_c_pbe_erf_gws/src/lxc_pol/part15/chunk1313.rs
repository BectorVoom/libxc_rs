//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1313/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1313<F: Float>(t51351: F, t9389: F, t4039: F, t9417: F, t9575: F, t9616: F, t4049: F, t9453: F, t2127: F, t3258: F, t850: F, t14093: F) -> (F, F, F, F, F, F) {
    let t54215 = t51351 * t9389;
    let t54217 = t4039 * t9417;
    let t54219 = t4039 * t9575;
    let t54224 = t51351 * t9616;
    let t54226 = t4049 * t9453;
    let t54230 = t850 * t3258 * t2127;
    let t54231 = t54230 * t14093;
    (t54215, t54217, t54219, t54224, t54226, t54231)
}
