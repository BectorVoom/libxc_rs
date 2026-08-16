//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 998/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk998(t2404: f64, t286: f64, t442: f64, t8132: f64, t670: f64, t327: f64, t7875: f64, t332: f64, t6: f64, t7877: f64, t2763: f64, t3326: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15489 = t2404 * t286;
    let t15491 = t8132 * t15489 * t442;
    let t15507 = t670 * t670;
    let t15508 = 1.0_f64 / t15507;
    let t15512 = t327 * t7875;
    let t15513 = t15512 * t332;
    let t15515 = t6 * t7877 * t442;
    let t15516 = t15513 * t15515;
    let t15541 = t3326 * t2763;
    (t15489, t15491, t15507, t15508, t15512, t15513, t15515, t15516, t15541)
}
