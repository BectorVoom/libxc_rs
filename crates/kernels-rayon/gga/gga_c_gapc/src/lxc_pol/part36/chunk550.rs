//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 550/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk550(t2920: f64, t3224: f64, t2536: f64, t2923: f64, t329: f64, t2164: f64, t996: f64, t493: f64, t876: f64, t1004: f64, t760: f64, t827: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
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
