//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1078/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1078<F: Float>(t513: F, t5842: F, t1570: F, t1576: F, t510: F, t5853: F, t131: F, t137: F, t5852: F, t1578: F, t1590: F, t133: F, t19295: F) -> (F, F, F, F, F, F, F) {
    let t19390 = t5842 * t513;
    let t19393 = t1570 * t1576;
    let t19398 = t510 * t5853;
    let t19407 = t131 / t5852 / t137;
    let t19408 = t1578 * t1578;
    let t19414 = t1590 * t1590;
    let t19420 = t133 * t19295;
    (t19390, t19393, t19398, t19407, t19408, t19414, t19420)
}
