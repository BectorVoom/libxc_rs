//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 756/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk756<F: Float>(t1: F, t282: F, t3: F, t311: F, t1944: F, t315: F, t3271: F, t871: F, t1018: F, t787: F, t1026: F, t2675: F, t2679: F, t2682: F, t3348: F, t1086: F, t2777: F) -> (F, F, F, F, F) {
    let t9471 = t282 * t1 * t3;
    let t9472 = t311 * t9471;
    let t9473 = t1944 * t315;
    let t9474 = t9472 * t9473;
    let t9476 = t871 * t3271;
    let t9477 = t1018 * t787;
    let t9478 = t9476 * t9477;
    let t9480 = t2675 * t1026;
    let t9481 = t9480 * t2679;
    let t9483 = t3348 * t2682;
    let t9485 = t1086 * t2777;
    (t9474, t9478, t9481, t9483, t9485)
}
