//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 977/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk977(t1165: f64, t1532: f64, t15407: f64, t3456: f64, t1487: f64, t435: f64, t3375: f64, t4987: f64, t1163: f64, t1586: f64, t4210: f64, t14575: f64, t540: f64) -> (f64, f64, f64, f64, f64) {
    let t15982 = t3456 * t1165 * t1532 * t15407;
    let t15995 = t435 * t1487;
    let t16008 = t3375 * t4987;
    let t16013 = t1163 * t1165 * t1586 * t4210;
    let t16017 = t1163 * t1165 * t540 * t14575;
    (t15982, t15995, t16008, t16013, t16017)
}
