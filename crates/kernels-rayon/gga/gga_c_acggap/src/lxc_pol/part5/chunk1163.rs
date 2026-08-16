//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1163/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1163(t1444: f64, t372: f64, t1449: f64, t322: f64, t1181: f64, t3361: f64, t4267: f64, t1163: f64, t1165: f64, t4210: f64, t5852: f64, t5574: f64, t997: f64) -> (f64, f64, f64, f64, f64) {
    let t20987 = t1444 * t372;
    let t20992 = t1449 * t322;
    let t20995 = t3361 * t1181 * t4267 * t20992;
    let t20999 = t1163 * t1165 * t5852 * t4210;
    let t21001 = t997 * t5574;
    (t20987, t20992, t20995, t20999, t21001)
}
