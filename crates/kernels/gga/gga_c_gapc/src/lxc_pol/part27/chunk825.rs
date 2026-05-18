//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 825/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk825<F: Float>(t3278: F, t3285: F, t3289: F, t3288: F, t7178: F, t1092: F, t3402: F, t9282: F, t3408: F, t612: F, t7451: F, t2545: F, t7453: F) -> (F, F, F, F, F, F, F) {
    let t9595 = t3278 * t3285;
    let t9597 = t3278 * t3289;
    let t9599 = t3288 * t7178;
    let t9600 = t1092 * t9599;
    let t9602 = t3402 * t9282;
    let t9603 = t9602 * t3408;
    let t9605 = t7451 * t612;
    let t9606 = t2545 * t7453;
    (t9595, t9597, t9599, t9600, t9603, t9605, t9606)
}
