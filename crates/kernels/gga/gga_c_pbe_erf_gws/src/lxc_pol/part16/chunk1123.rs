//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1123/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1123<F: Float>(t14023: F, t14548: F, t863: F, t14547: F, t28029: F, t6523: F, t14031: F, t9556: F, t14011: F, t9344: F, t850: F, t852: F, t9441: F, t51325: F, t14058: F, t3279: F) -> (F, F, F, F, F, F) {
    let t54329 = t863 * t14023 * t14548;
    let t54333 = t14547 * t6523 * t28029;
    let t54335 = t14031 * t9556;
    let t54338 = t14011 * t9344;
    let t54341 = t850 * t9441 * t852;
    let t54342 = t54341 * t51325;
    let t54344 = t14058 * t3279;
    (t54329, t54333, t54335, t54338, t54342, t54344)
}
