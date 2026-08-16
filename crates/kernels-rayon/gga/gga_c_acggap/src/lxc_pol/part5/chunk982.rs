//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 982/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk982(t322: f64, t4199: f64, t1165: f64, t13585: f64, t1532: f64, t329: f64, t56: f64, t2029: f64, t4258: f64, t1008: f64, t5237: f64, t14283: f64, t537: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16171 = t4199 * t322;
    let t16174 = t13585 * t1165 * t1532 * t16171;
    let t16183 = t329 * t56;
    let t16184 = t16183 * t2029;
    let t16185 = t16184 * t4258;
    let t16191 = t1008 * t5237;
    let t16203 = t14283 * t537;
    (t16171, t16174, t16183, t16184, t16185, t16191, t16203)
}
