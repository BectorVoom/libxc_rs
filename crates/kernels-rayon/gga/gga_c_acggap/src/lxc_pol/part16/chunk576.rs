//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 576/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk576(t1576: f64, t997: f64, t4210: f64, t535: f64, t1181: f64, t1163: f64, t1165: f64, t530: f64, t3194: f64, t540: f64, t1005: f64, t1423: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4957 = 0.40015750243531754508e-2_f64 * t997 * t1576;
    let t4958 = t535 * t4210;
    let t4959 = t1181 * t4958;
    let t4961 = 0.85748036236139473944e-3_f64 * t1163 * t4959;
    let t4967 = t1165 * t530 * t4210;
    let t4969 = 0.17149607247227894789e-2_f64 * t3194 * t4967;
    let t4987 = t1165 * t540 * t4210;
    let t4989 = 0.85748036236139473944e-3_f64 * t1163 * t4987;
    let t4994 = t1005 * t1423;
    (t4957, t4959, t4961, t4967, t4969, t4987, t4989, t4994)
}
