//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 552/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk552<F: Float>(t3411: F, t3415: F, t1084: F, t3127: F, t2664: F, t2660: F, t3132: F, t129: F, t2520: F, t1078: F, t197: F, t2493: F, t1077: F, t1018: F, t916: F, t3096: F, t919: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3416 = t3411 * t3415;
    let t3418 = t1084 * t3127;
    let t3419 = t3418 * t2664;
    let t3421 = t2660 * t3132;
    let t3422 = t3421 * t2664;
    let t3424 = t2520 * t129;
    let t3425 = t3424 * t1078;
    let t3427 = t197 * t2493;
    let t3428 = t1077 * t3427;
    let t3430 = t916 * t1018;
    let t3431 = t3096 * t919;
    (t3416, t3418, t3419, t3421, t3422, t3424, t3425, t3427, t3428, t3430, t3431)
}
