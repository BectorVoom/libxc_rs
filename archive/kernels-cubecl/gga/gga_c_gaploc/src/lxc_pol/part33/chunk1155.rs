//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1155/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1155<F: Float>(t1406: F, t6575: F, t9264: F, t2349: F, t2482: F, t9263: F, t165: F, t4324: F, t874: F, t4812: F, t6583: F, t161: F, t20535: F, t4130: F) -> (F, F, F, F, F) {
    let t31356 = t1406 * t6575;
    let t31357 = t31356 * t9264;
    let t31360 = t9263 * t2349 * t2482;
    let t31379 = t165 * t874 * t4324;
    let t31382 = F::cast_from(0.38342925953920749676e1_f64) * t6583 * t4812 * t31379;
    let t31386 = F::cast_from(0.23005755572352449806e1_f64) * t20535 * t4130 * t161 * t31379;
    (t31356, t31357, t31360, t31382, t31386)
}
