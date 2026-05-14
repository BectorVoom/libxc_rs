//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 836/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk836<F: Float>(t31028: F, t7380: F, t1061: F, t429: F, t130: F, t1964: F, t2037: F, t377: F, t7684: F, t409: F, t7712: F, t957: F, t1181: F, t30806: F, t3491: F, t599: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31029 = t7380 * t31028;
    let t31032 = t429 * t1061;
    let t31033 = t7380 * t31032;
    let t31035 = t130 * t1964;
    let t31036 = t31035 * t2037;
    let t31038 = t377 * t7684;
    let t31039 = t31038 * t409;
    let t31041 = t7712 * t957;
    let t31045 = t30806 * t1181 * t599 * t3491;
    (t31029, t31032, t31033, t31035, t31036, t31038, t31039, t31041, t31045)
}
