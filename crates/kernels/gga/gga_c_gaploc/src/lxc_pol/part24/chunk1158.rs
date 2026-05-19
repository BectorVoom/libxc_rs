//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1158/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1158<F: Float>(t31346: F, t1406: F, t6575: F, t9264: F, t2349: F, t2482: F, t9263: F, t165: F, t4324: F, t874: F, t4812: F, t6583: F) -> (F, F, F, F, F, F) {
    let t31347 = F::cast_from(0.76685851907841499352e0_f64) * t31346;
    let t31356 = t1406 * t6575;
    let t31357 = t31356 * t9264;
    let t31358 = F::cast_from(0.1533717038156829987e1_f64) * t31357;
    let t31360 = t9263 * t2349 * t2482;
    let t31361 = F::cast_from(0.1533717038156829987e1_f64) * t31360;
    let t31379 = t165 * t874 * t4324;
    let t31382 = F::cast_from(0.38342925953920749676e1_f64) * t6583 * t4812 * t31379;
    (t31347, t31356, t31358, t31361, t31379, t31382)
}
