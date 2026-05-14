//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1207/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1207<F: Float>(t26278: F, t26289: F, t26300: F, t26306: F, t26309: F, t26419: F, t26425: F, t26428: F, t26430: F, t26433: F, t26435: F, t26443: F, t26446: F, t26448: F, t26508: F, t26523: F, t26539: F) -> (F,) {
    let t26554 = 0.49671e0 * t26419 - 0.485484375e1 * t26425 - 0.247573125e0 * t26428 + 0.3300975e0 * t26430 + 0.11651625e2 * t26433 - 0.51785e1 * t26435 - 0.20128333333333333334e1 * t26278 + 0.72462e1 * t26289 - 0.108693e2 * t26300 - 0.24154e1 * t26306 + 0.80513333333333333333e0 * t26309 + 0.99342e0 * t26443 - 0.298026e1 * t26446 + 0.16504875e0 * t26448;
    let t26556 = t26508 + t26523 + t26539 + t26554;
    (t26556,)
}
