//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 993/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk993(t1165: f64, t14187: f64, t15407: f64, t530: f64, t322: f64, t4162: f64, t1532: f64, t3451: f64, t12816: f64, t4267: f64, t4282: f64, t955: f64) -> (f64, f64, f64, f64, f64) {
    let t16537 = t14187 * t1165 * t530 * t15407;
    let t16539 = t4162 * t322;
    let t16542 = t3451 * t1165 * t1532 * t16539;
    let t16546 = t4282 * t1165 * t4267 * t12816;
    let t16548 = t955 * t322;
    (t16537, t16539, t16542, t16546, t16548)
}
