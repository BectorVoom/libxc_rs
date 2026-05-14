//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 756/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk756<F: Float>(t9932: F, t9934: F, t3434: F, t949: F, t2749: F, t3348: F, t3322: F, t9414: F, t3330: F, t9418: F, t3418: F, t7522: F, t3421: F, t1736: F, t291: F, t7949: F, t959: F) -> (F, F, F, F, F, F, F, F) {
    let t9935 = t9932 * t9934;
    let t9937 = t3434 * t949;
    let t9939 = t3348 * t2749;
    let t9941 = t9414 * t3322;
    let t9944 = t9418 * t3330;
    let t9946 = t3418 * t7522;
    let t9948 = t3421 * t7522;
    let t9950 = t1736 * t291;
    let t9952 = t9950 * t959 * t7949;
    (t9935, t9937, t9939, t9941, t9944, t9946, t9948, t9952)
}
