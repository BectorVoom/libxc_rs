//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 503/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk503(t1287: f64, t2120: f64, t1291: f64, t2144: f64, t115: f64, t2010: f64, t155: f64, t2156: f64, t635: f64, t1294: f64, t2164: f64, t1278: f64, t2182: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3471 = t2120 * t1287;
    let t3489 = t2144 * t1291;
    let t3491 = t2010 * t115;
    let t3500 = t155 * t2156;
    let t3501 = t3500 * t635;
    let t3504 = t2164 * t1294;
    let t3517 = t2182 * t1278;
    (t3471, t3489, t3491, t3500, t3501, t3504, t3517)
}
