//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1211/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1211(t2323: f64, t6513: f64, t1: f64, t6382: f64, t253: f64, t20500: f64, t2113: f64, t21400: f64, t21482: f64, t21494: f64, t21495: f64, t21502: f64, t21508: f64, t21513: f64, t21514: f64, t2255: f64, t2312: f64, t2343: f64, t6211: f64, t6275: f64, t6278: f64, t851: f64, t875: f64) -> f64 {
    let t21516 = t2323 * t6513;
    let t21518 = t6382 * t1;
    let t21519 = t21518 * t253;
    let t21524 = -t2312 * t2255 * t851 * t21482 / 96.0_f64 - t2312 * t2255 * t2113 * t6211 / 48.0_f64 - t21494 + 7.0_f64 / 288.0_f64 * t21495 - t21502 + t6275 * t20500 * t6278 / 16.0_f64 + 595.0_f64 / 648.0_f64 * t21508 + t21513 - 119.0_f64 / 576.0_f64 * t21514 + 7.0_f64 / 288.0_f64 * t21516 + 5.0_f64 / 16.0_f64 * t2343 * t21519 * t21400 * t875;
    t21524
}
