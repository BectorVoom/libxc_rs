//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1074/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1074<F: Float>(t21507: F, t935: F, t6045: F, t855: F, t863: F, t888: F, t2327: F, t6505: F, t2323: F, t6513: F, t1: F, t6382: F, t253: F, t20500: F, t2113: F, t21400: F, t21482: F, t21494: F, t21495: F, t21502: F, t2255: F, t2312: F, t2343: F, t6211: F, t6275: F, t6278: F, t851: F, t875: F) -> (F, F) {
    let t21508 = t21507 * t935;
    let t21511 = t863 * t855 * t6045;
    let t21512 = t21511 * t888;
    let t21513 = 455.0 / 162.0 * t21512;
    let t21514 = t6505 * t2327;
    let t21516 = t2323 * t6513;
    let t21518 = t6382 * t1;
    let t21519 = t21518 * t253;
    let t21524 = -t2312 * t2255 * t851 * t21482 / 96.0 - t2312 * t2255 * t2113 * t6211 / 48.0 - t21494 + 7.0 / 288.0 * t21495 - t21502 + t6275 * t20500 * t6278 / 16.0 + 595.0 / 648.0 * t21508 + t21513 - 119.0 / 576.0 * t21514 + 7.0 / 288.0 * t21516 + 5.0 / 16.0 * t2343 * t21519 * t21400 * t875;
    (t21513, t21524)
}
