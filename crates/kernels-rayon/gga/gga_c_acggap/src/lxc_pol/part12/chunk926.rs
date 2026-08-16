//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 926/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk926(t2090: f64, t4210: f64, t15386: f64, t31057: f64, t1998: f64, t3348: f64, t7447: f64, t7808: f64, t7440: f64, t7812: f64, t30402: f64, t30407: f64, t30409: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31058 = t2090 * t4210;
    let t31060 = t31057 * t15386 * t31058;
    let t31074 = t1998 * t3348;
    let t31081 = t7447 * t7808;
    let t31083 = t7440 * t7812;
    let t31095 = t30407 * t30402 * t30409 * t372;
    (t31058, t31060, t31074, t31081, t31083, t31095)
}
