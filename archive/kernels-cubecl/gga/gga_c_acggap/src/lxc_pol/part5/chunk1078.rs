//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1078/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1078<F: Float>(t3378: F, t4173: F, t13259: F, t1630: F, t1160: F, t1539: F, t18973: F, t4166: F, t4210: F, t4146: F, t16020: F, t1629: F) -> (F, F, F, F, F, F) {
    let t19237 = t3378 * t4173;
    let t19240 = t13259 * t1630;
    let t19243 = t1160 * t18973 * t1539;
    let t19246 = t1160 * t4166 * t4210;
    let t19249 = t1160 * t4146 * t4210;
    let t19252 = t1160 * t1629 * t16020;
    (t19237, t19240, t19243, t19246, t19249, t19252)
}
