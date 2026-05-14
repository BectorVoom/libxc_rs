//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 836/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk836<F: Float>(t2090: F, t4210: F, t15386: F, t31057: F, t1998: F, t3348: F, t7447: F, t7808: F, t7440: F, t7812: F, t30402: F, t30407: F, t30409: F, t372: F, t141: F, t7335: F) -> (F, F, F, F, F, F, F) {
    let t31058 = t2090 * t4210;
    let t31060 = t31057 * t15386 * t31058;
    let t31074 = t1998 * t3348;
    let t31081 = t7447 * t7808;
    let t31083 = t7440 * t7812;
    let t31095 = t30407 * t30402 * t30409 * t372;
    let t31097 = t7335 * t141;
    (t31058, t31060, t31074, t31081, t31083, t31095, t31097)
}
