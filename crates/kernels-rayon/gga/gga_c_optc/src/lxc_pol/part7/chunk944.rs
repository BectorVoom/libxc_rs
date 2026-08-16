//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 944/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk944(t8527: f64, t8979: f64, t1146: f64, t3160: f64, t1141: f64, t3169: f64, t1145: f64, t454: f64, t1182: f64, t3171: f64, t3264: f64, t2367: f64, t3224: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8980 = t8527 + t8979;
    let t8984 = t3160 * t1146;
    let t8988 = t1141 * t3169;
    let t8995 = t1145 * t1145;
    let t8996 = 1.0_f64 / t8995;
    let t8997 = t454 * t8996;
    let t8998 = t3171 * t1182;
    let t9002 = t3169 * t1182;
    let t9003 = t9002 * t3264;
    let t9006 = t2367 * t3224;
    (t8980, t8984, t8988, t8995, t8996, t8997, t8998, t9002, t9003, t9006)
}
