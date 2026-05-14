//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1202/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1202<F: Float>(t1045: F, t26346: F, t26278: F, t26289: F, t26300: F, t26306: F, t26309: F, t26419: F, t26425: F, t26428: F, t26430: F, t26433: F, t26435: F, t26443: F, t26446: F, t1037: F, t1056: F, t26360: F, t26391: F, t26417: F) -> (F, F) {
    let t26448 = t1045 * t26346;
    let t26450 = 0.49293999999999999999e0 * t26419 - 0.3560484375e1 * t26425 - 0.46074375e0 * t26428 + 0.614325e0 * t26430 + 0.85451625e1 * t26433 - 0.379785e1 * t26435 - 0.19931111111111111111e1 * t26278 + 0.71752000000000000001e1 * t26289 - 0.107628e2 * t26300 - 0.23917333333333333333e1 * t26306 + 0.79724444444444444444e0 * t26309 + 0.98587999999999999999e0 * t26443 - 0.295764e1 * t26446 + 0.3071625e0 * t26448;
    let t26455 = 1.0 * t1037 * (t26360 + t26391 + t26417 + t26450) * t1056;
    (t26448, t26455)
}
