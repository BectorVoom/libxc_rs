//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 906/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk906<F: Float>(t1163: F, t1165: F, t1532: F, t16548: F, t3372: F, t4959: F, t1181: F, t535: F, t864: F, t944: F, t406: F, t12801: F, t3382: F, t4364: F, t5133: F, t4326: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16551 = t1163 * t1165 * t1532 * t16548;
    let t16553 = t3372 * t4959;
    let t16557 = t1163 * t1181 * t535 * t16548;
    let t16559 = t944 * t864;
    let t16560 = t16559 * t406;
    let t16563 = t12801 * t1165 * t1532 * t16560;
    let t16569 = t3382 * t4364;
    let t16575 = t3372 * t5133;
    let t16602 = t3372 * t4326;
    (t16551, t16553, t16557, t16559, t16560, t16563, t16569, t16575, t16602)
}
