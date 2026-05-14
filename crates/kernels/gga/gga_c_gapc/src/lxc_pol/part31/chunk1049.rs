//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1049/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1049<F: Float>(t1734: F, t24759: F, t1084: F, t29654: F, t15680: F, t26312: F, t3402: F, t11966: F, t28346: F, t189: F, t1899: F, t15508: F, t90: F, t18680: F, t277: F, t327: F) -> (F, F, F, F, F, F, F) {
    let t34186 = t1734 * t24759;
    let t34188 = t1084 * t34186 * t29654;
    let t34191 = t3402 * t26312 * t15680;
    let t34193 = t11966 * t28346;
    let t34195 = t189 * t1899;
    let t34197 = t15508 * t90;
    let t34200 = t277 * t34195 * t34197 * t327 * t18680;
    (t34186, t34188, t34191, t34193, t34195, t34197, t34200)
}
