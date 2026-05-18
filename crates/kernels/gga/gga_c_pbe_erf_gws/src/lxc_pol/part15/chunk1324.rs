//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1324/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1324<F: Float>(t14011: F, t9344: F, t850: F, t852: F, t9441: F, t51325: F, t14058: F, t3279: F, t4049: F, t9647: F, t4028: F, t9009: F) -> (F, F, F, F, F) {
    let t54338 = t14011 * t9344;
    let t54341 = t850 * t9441 * t852;
    let t54342 = t54341 * t51325;
    let t54344 = t14058 * t3279;
    let t54345 = F::new(35.0) / F::new(288.0) * t54344;
    let t54346 = t4049 * t9647;
    let t54348 = t4028 * t9009;
    (t54338, t54342, t54345, t54346, t54348)
}
