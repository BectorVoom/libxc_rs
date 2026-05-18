//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 959/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk959<F: Float>(t3181: F, t439: F, t442: F, t446: F, t8113: F, t19: F, t8915: F, t123: F, t9129: F, t4434: F) -> (F, F, F, F, F, F, F, F) {
    let t9166 = F::new(1.0) / t3181 / t439;
    let t9167 = t9166 * t442;
    let t9168 = t9167 * t446;
    let t9169 = t9168 * t8113;
    let t9170 = t8915 * t19;
    let t9171 = t9170 * t123;
    let t9172 = t9129 * t9171;
    let t9175 = t4434 * t8113;
    (t9166, t9167, t9168, t9169, t9170, t9171, t9172, t9175)
}
