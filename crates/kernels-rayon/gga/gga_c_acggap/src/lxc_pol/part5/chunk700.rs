//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 700/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk700(t1181: f64, t4982: f64, t1165: f64, t4210: f64, t540: f64, t1163: f64, t3169: f64, t1005: f64, t1423: f64, t3765: f64, t527: f64, t398: f64, t525: f64, t966: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4983 = t1181 * t4982;
    let t4987 = t1165 * t540 * t4210;
    let t4989 = 0.85748036236139473944e-3_f64 * t1163 * t4987;
    let t4991 = t1181 * t540 * t3169;
    let t4994 = t1005 * t1423;
    let t4996 = t3765 * t527;
    let t4999 = t398 * t966 * t525;
    (t4983, t4987, t4989, t4991, t4994, t4996, t4999)
}
