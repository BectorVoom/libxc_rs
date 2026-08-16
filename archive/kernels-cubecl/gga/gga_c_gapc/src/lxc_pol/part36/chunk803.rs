//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 803/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk803<F: Float>(t1092: F, t9599: F, t3402: F, t9282: F, t3408: F, t612: F, t7451: F, t2545: F, t7453: F, t197: F, t7776: F, t1077: F) -> (F, F, F, F) {
    let t9600 = t1092 * t9599;
    let t9602 = t3402 * t9282;
    let t9603 = t9602 * t3408;
    let t9605 = t7451 * t612;
    let t9606 = t2545 * t7453;
    let t9607 = t9605 * t9606;
    let t9609 = t197 * t7776;
    let t9610 = t1077 * t9609;
    (t9600, t9603, t9607, t9610)
}
