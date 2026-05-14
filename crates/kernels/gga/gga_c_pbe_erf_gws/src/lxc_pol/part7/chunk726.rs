//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 726/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk726<F: Float>(t2171: F, t2345: F, t6282: F, t2157: F, t810: F, t2113: F, t2257: F, t2255: F, t745: F, t874: F, t343: F, t851: F, t2189: F, t274: F, t6: F, t3235: F, t875: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6284 = t2345 * t6282 * t2171;
    let t6287 = t2157 * t810;
    let t6289 = t2345 * t6282 * t6287;
    let t6292 = t2113 * t2257;
    let t6293 = t2255 * t6292;
    let t6296 = t745 * t874;
    let t6297 = t6296 * t343;
    let t6298 = t851 * t6297;
    let t6299 = t2255 * t6298;
    let t6303 = t274 * t2189 * t343;
    let t6304 = t851 * t6303;
    let t6305 = t2255 * t6304;
    let t6308 = t6 * t2189;
    let t6310 = t3235 * t6308 * t875;
    (t6284, t6287, t6289, t6293, t6297, t6299, t6303, t6305, t6308, t6310)
}
