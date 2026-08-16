//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 995/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk995<F: Float>(t3382: F, t4364: F, t3372: F, t5133: F, t4326: F, t14220: F, t4916: F, t4389: F, t4393: F, t4567: F, t1165: F, t3451: F, t4183: F, t4289: F) -> (F, F, F, F, F, F, F) {
    let t16569 = t3382 * t4364;
    let t16575 = t3372 * t5133;
    let t16602 = t3372 * t4326;
    let t16608 = t14220 * t4916;
    let t16610 = t4389 * t4393;
    let t16612 = t4389 * t4567;
    let t16625 = t3451 * t1165 * t4289 * t4183;
    (t16569, t16575, t16602, t16608, t16610, t16612, t16625)
}
