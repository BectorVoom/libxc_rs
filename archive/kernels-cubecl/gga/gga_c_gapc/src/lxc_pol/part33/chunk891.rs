//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 891/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk891<F: Float>(t10336: F, t3227: F, t297: F, t493: F, t7371: F, t3217: F, t3224: F, t8350: F, t268: F, t2920: F) -> (F, F, F, F, F) {
    let t10337 = t10336 * t3227;
    let t10339 = t493 * t297;
    let t10340 = t10339 * t7371;
    let t10341 = t3217 * t10340;
    let t10343 = t8350 * t3224;
    let t10344 = t10343 * t3227;
    let t10346 = t2920 * t268;
    (t10337, t10341, t10343, t10344, t10346)
}
