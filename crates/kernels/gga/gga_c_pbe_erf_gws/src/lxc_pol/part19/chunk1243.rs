//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1243/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1243<F: Float>(t14046: F, t3184: F, t51408: F, t3148: F, t14023: F, t14548: F, t863: F, t51412: F, t14058: F, t3279: F, t1158: F, t51395: F) -> (F, F, F, F, F, F, F) {
    let t54319 = t14046 * t3184;
    let t54321 = F::new(35.0) / F::new(216.0) * t51408;
    let t54322 = t14046 * t3148;
    let t54329 = t863 * t14023 * t14548;
    let t54331 = F::new(35.0) / F::new(108.0) * t51412;
    let t54344 = t14058 * t3279;
    let t54352 = t51395 * t1158;
    (t54319, t54321, t54322, t54329, t54331, t54344, t54352)
}
