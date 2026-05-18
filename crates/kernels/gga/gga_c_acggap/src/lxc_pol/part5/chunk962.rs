//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 962/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk962<F: Float>(t13293: F, t1432: F, t15386: F, t3176: F, t1: F, t1161: F, t3401: F, t1160: F, t1421: F, t176: F, t3196: F, t322: F, t945: F) -> (F, F, F, F, F) {
    let t15389 = t13293 * t15386 * t1432 * t3176;
    let t15392 = t1161 * t3401 * t1;
    let t15393 = t1160 * t15392;
    let t15396 = t15393 * t176 * t1421 * t3196;
    let t15407 = t945 * t322;
    (t15389, t15392, t15393, t15396, t15407)
}
