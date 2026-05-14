//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 773/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk773<F: Float>(t1106: F, t372: F, t322: F, t955: F, t361: F, t435: F, t171: F, t3300: F, t1164: F, t4847: F, t1298: F, t467: F, t1410: F, t407: F, t406: F, t6263: F) -> (F, F, F, F, F, F, F, F) {
    let t16507 = t1106 * t372;
    let t16548 = t955 * t322;
    let t17752 = t361 * t435;
    let t17912 = t171 * t3300;
    let t17972 = t1164 * t4847;
    let t19409 = t1298 * t467;
    let t19834 = t407 * t1410;
    let t20138 = t6263 * t406;
    (t16507, t16548, t17752, t17912, t17972, t19409, t19834, t20138)
}
