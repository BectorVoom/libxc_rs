//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1211/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1211<F: Float>(t2323: F, t6513: F, t1: F, t6382: F, t253: F, t20500: F, t2113: F, t21400: F, t21482: F, t21494: F, t21495: F, t21502: F, t21508: F, t21513: F, t21514: F, t2255: F, t2312: F, t2343: F, t6211: F, t6275: F, t6278: F, t851: F, t875: F) -> F {
    let t21516 = t2323 * t6513;
    let t21518 = t6382 * t1;
    let t21519 = t21518 * t253;
    let t21524 = -t2312 * t2255 * t851 * t21482 / F::new(96.0) - t2312 * t2255 * t2113 * t6211 / F::new(48.0) - t21494 + F::new(7.0) / F::new(288.0) * t21495 - t21502 + t6275 * t20500 * t6278 / F::new(16.0) + F::new(595.0) / F::new(648.0) * t21508 + t21513 - F::new(119.0) / F::new(576.0) * t21514 + F::new(7.0) / F::new(288.0) * t21516 + F::new(5.0) / F::new(16.0) * t2343 * t21519 * t21400 * t875;
    t21524
}
