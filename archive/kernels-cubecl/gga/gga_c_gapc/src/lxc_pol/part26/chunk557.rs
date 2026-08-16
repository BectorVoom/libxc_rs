//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 557/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk557<F: Float>(t2920: F, t3224: F, t2536: F, t2923: F, t329: F, t2164: F, t996: F, t493: F, t876: F, t1004: F, t760: F, t827: F) -> (F, F, F, F, F, F, F, F) {
    let t3225 = t2920 * t3224;
    let t3227 = t2923 * t329 * t2536;
    let t3228 = t3225 * t3227;
    let t3230 = t996 * t2164;
    let t3231 = t493 * t876;
    let t3232 = t3230 * t3231;
    let t3234 = t1004 * t760;
    let t3235 = t3234 * t827;
    (t3225, t3227, t3228, t3230, t3231, t3232, t3234, t3235)
}
