//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1416/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1416(t35173: f64, t35184: f64, t35186: f64, t35188: f64, t35197: f64, t35200: f64, t35177: f64, t35182: f64, t35190: f64, t35192: f64, t37223: f64, t35205: f64) -> (f64, f64) {
    let t37224 = 0.63350674672043801542e-5_f64 * t35173;
    let t37227 = 0.69504740211613770836e-3_f64 * t35184;
    let t37228 = 0.34752370105806885418e-3_f64 * t35186;
    let t37229 = 0.34782544165564226085e-4_f64 * t35188;
    let t37232 = 0.12144921875e-2_f64 * t35197;
    let t37233 = 0.14232178796075385434e-7_f64 * t35200;
    let t37234 = t37223 + t37224 - 0.10925861285174334493e-8_f64 * t35177 - 0.38527756621470067412e-7_f64 * t35182 - t37227 - t37228 + t37229 - 0.31433990684987949195e-7_f64 * t35190 - 0.67632724766374884053e-5_f64 * t35192 + t37232 - t37233;
    let t37236 = 0.2748593934505475288e-5_f64 * t35205;
    (t37234, t37236)
}
