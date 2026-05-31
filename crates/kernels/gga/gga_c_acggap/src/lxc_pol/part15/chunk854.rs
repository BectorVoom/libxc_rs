//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 854/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk854<F: Float>(t166: F, t117: F, t3033: F, t130: F, t972: F, t182: F, t851: F, t1015: F, t173: F, t157: F, t406: F, t879: F) -> (F, F, F, F, F, F) {
    let t13461 = t166 * t166;
    let t13462 = F::cast_from(1.0_f64) / t13461;
    let t13483 = F::cast_from(1.0_f64) / t3033 / t117;
    let t13716 = t130 * t972;
    let t14046 = t851 * t182;
    let t14423 = F::cast_from(1.0_f64) / t1015 / t173;
    let t14575 = t879 * t406 * t157;
    (t13462, t13483, t13716, t14046, t14423, t14575)
}
