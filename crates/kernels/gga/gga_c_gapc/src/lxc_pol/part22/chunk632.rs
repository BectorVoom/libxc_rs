//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 632/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk632<F: Float>(t1881: F, t190: F, t1033: F, t198: F, t5: F, t681: F, t19: F, t147: F, t203: F, t144: F, t1: F, t457: F, t350: F, t676: F, t186: F, t632: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5294 = t1881 * t190;
    let t5296 = t1033 * t198;
    let t5298 = t681 * t5;
    let t5311 = t5 * t19;
    let t5312 = t5311 * t147;
    let t5319 = t203 * t5;
    let t5325 = t1033 * t144;
    let t5390 = t457 * t1;
    let t5391 = t5390 * t350;
    let t5392 = t676 * t5391;
    let t5395 = t632 * t186;
    (t5294, t5296, t5298, t5312, t5319, t5325, t5390, t5391, t5392, t5395)
}
