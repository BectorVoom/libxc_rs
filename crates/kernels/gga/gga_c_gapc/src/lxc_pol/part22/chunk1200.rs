//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1200/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1200<F: Float>(t35197: F, t35200: F, t35177: F, t35182: F, t35190: F, t35192: F, t37223: F, t37224: F, t37227: F, t37228: F, t37229: F, t35205: F, t35210: F, t35212: F, t35215: F, t35217: F) -> (F, F, F, F, F, F) {
    let t37232 = 0.12144921875e-2 * t35197;
    let t37233 = 0.14232178796075385434e-7 * t35200;
    let t37234 = t37223 + t37224 - 0.10925861285174334493e-8 * t35177 - 0.38527756621470067412e-7 * t35182 - t37227 - t37228 + t37229 - 0.31433990684987949195e-7 * t35190 - 0.67632724766374884053e-5 * t35192 + t37232 - t37233;
    let t37236 = 0.2748593934505475288e-5 * t35205;
    let t37237 = 0.18477280112679442116e-5 * t35210;
    let t37238 = 0.80192315782160920384e-6 * t35212;
    let t37239 = 0.80045999977926802214e-7 * t35215;
    let t37240 = 0.10298285674687440379e-4 * t35217;
    (t37234, t37236, t37237, t37238, t37239, t37240)
}
