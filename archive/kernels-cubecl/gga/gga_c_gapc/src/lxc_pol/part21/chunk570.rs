//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 570/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk570<F: Float>(t1072: F, t1074: F, t869: F, t3272: F, t1069: F, t910: F, t1084: F, t3072: F, t1087: F, t3074: F, t2636: F) -> (F, F, F, F, F, F) {
    let t3314 = t869 * t1072 * t1074;
    let t3316 = t3272 * t1074;
    let t3318 = t1069 * t910;
    let t3320 = t1084 * t3072;
    let t3321 = t1087 * t3074;
    let t3322 = t2636 * t3321;
    (t3314, t3316, t3318, t3320, t3321, t3322)
}
