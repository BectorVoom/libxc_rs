//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 963/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk963<F: Float>(t513: F, t5842: F, t1570: F, t1576: F, t510: F, t5853: F, t131: F, t137: F, t5852: F, t1578: F, t1590: F, t133: F, t19295: F, t19298: F, t19301: F, t19216: F, t19219: F, t19229: F, t19236: F, t19240: F, t19249: F, t19264: F, t19282: F, t19286: F, t19290: F, t19294: F, t19312: F) -> (F, F, F, F, F, F, F) {
    let t19390 = t5842 * t513;
    let t19393 = t1570 * t1576;
    let t19398 = t510 * t5853;
    let t19407 = t131 / t5852 / t137;
    let t19408 = t1578 * t1578;
    let t19414 = t1590 * t1590;
    let t19420 = t133 * t19295;
    let t19422 = t133 * t19298;
    let t19424 = t133 * t19301;
    let t19426 = -t19216 + t19219 + t19229 - t19236 - t19240 - t19249 - t19264 + t19282 + t19286 + t19290 + t19294 + t19312 + 0.7152465185185185185e1 * t19420 - 0.45980133333333333333e1 * t19422 + 0.22990066666666666667e1 * t19424;
    (t19390, t19393, t19398, t19407, t19408, t19414, t19426)
}
