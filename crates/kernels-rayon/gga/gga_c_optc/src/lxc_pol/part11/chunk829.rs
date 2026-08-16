//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 829/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk829(t1156: f64, t5393: f64, t3212: f64, t1128: f64, t5412: f64, t3192: f64, t5407: f64, t3186: f64, t1148: f64, t5275: f64, t911: f64, t1168: f64, t5279: f64, t871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16001 = t1156 * t5393;
    let t16002 = t3212 * t16001;
    let t16004 = t1128 * t5412;
    let t16005 = t3192 * t16004;
    let t16007 = t1128 * t5407;
    let t16008 = t3186 * t16007;
    let t16011 = t1148 * t5275 * t911;
    let t16024 = t1168 * t5279 * t871;
    (t16001, t16002, t16004, t16005, t16007, t16008, t16011, t16024)
}
