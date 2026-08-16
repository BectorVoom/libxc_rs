//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 912/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk912(t11287: f64, t3659: f64, t4908: f64, t687: f64, t4915: f64, t1049: f64, t3179: f64, t1616: f64, t1611: f64, t3721: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11288 = 2.0_f64 * t11287;
    let t11289 = t4908 * t3659;
    let t11290 = 2.0_f64 * t11289;
    let t11291 = t3659 * t687;
    let t11292 = t4915 * t11291;
    let t11293 = 6.0_f64 * t11292;
    let t11294 = t1049 * t3179;
    let t11295 = t1616 * t11294;
    let t11296 = 4.0_f64 * t11295;
    let t11297 = t1611 * t3721;
    let t11298 = t3721 * t687;
    let t11299 = t1616 * t11298;
    (t11288, t11289, t11290, t11291, t11292, t11293, t11294, t11295, t11296, t11297, t11298, t11299)
}
