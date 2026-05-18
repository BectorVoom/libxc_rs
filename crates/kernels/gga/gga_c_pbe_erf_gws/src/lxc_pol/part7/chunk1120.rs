//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1120/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1120<F: Float>(t19777: F, t4390: F, t2271: F, t4383: F, t822: F, t19745: F, t2306: F, t3074: F, t19751: F, t2118: F, t2382: F, t4384: F) -> (F, F, F, F, F) {
    let t20110 = t19777 * t4390;
    let t20112 = t2271 * t4383;
    let t20113 = t822 * t20112;
    let t20117 = t3074 * t2306 * t19745;
    let t20121 = t2382 * t2118 * t19751;
    let t20124 = t2382 * t4384;
    (t20110, t20113, t20117, t20121, t20124)
}
