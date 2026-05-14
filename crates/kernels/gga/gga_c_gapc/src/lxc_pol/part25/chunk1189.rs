//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1189/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1189<F: Float>(t35162: F, t35169: F, t35173: F, t35184: F, t35186: F, t35188: F, t35197: F, t35200: F, t35205: F, t35210: F, t35212: F, t35215: F, t35217: F, t35222: F, t35225: F, t35228: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37220 = 0.80192315782160920384e-6 * t35162;
    let t37223 = 0.11984097313886885523e-6 * t35169;
    let t37224 = 0.63350674672043801542e-5 * t35173;
    let t37227 = 0.69504740211613770836e-3 * t35184;
    let t37228 = 0.34752370105806885418e-3 * t35186;
    let t37229 = 0.34782544165564226085e-4 * t35188;
    let t37232 = 0.12144921875e-2 * t35197;
    let t37233 = 0.14232178796075385434e-7 * t35200;
    let t37236 = 0.2748593934505475288e-5 * t35205;
    let t37237 = 0.18477280112679442116e-5 * t35210;
    let t37238 = 0.80192315782160920384e-6 * t35212;
    let t37239 = 0.80045999977926802214e-7 * t35215;
    let t37240 = 0.10298285674687440379e-4 * t35217;
    let t37241 = 0.15018333275585850553e-5 * t35222;
    let t37242 = 0.6070699179094394313e-6 * t35225;
    let t37243 = 0.43440462632258606772e-4 * t35228;
    (t37220, t37223, t37224, t37227, t37228, t37229, t37232, t37233, t37236, t37237, t37238, t37239, t37240, t37241, t37242, t37243)
}
