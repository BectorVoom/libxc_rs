//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 699/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk699(t1163: f64, t4959: f64, t1165: f64, t3196: f64, t540: f64, t4210: f64, t530: f64, t3194: f64, t1181: f64, t3169: f64, t535: f64, t3176: f64, t4643: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4961 = 0.85748036236139473944e-3_f64 * t1163 * t4959;
    let t4963 = t1165 * t540 * t3196;
    let t4967 = t1165 * t530 * t4210;
    let t4969 = 0.17149607247227894789e-2_f64 * t3194 * t4967;
    let t4971 = t1165 * t530 * t3196;
    let t4975 = t1181 * t530 * t3169;
    let t4978 = t535 * t3196;
    let t4979 = t1181 * t4978;
    let t4982 = t4643 * t3176;
    (t4961, t4963, t4967, t4969, t4971, t4975, t4979, t4982)
}
