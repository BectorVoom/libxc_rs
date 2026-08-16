//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 655/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk655(t1008: f64, t1576: f64, t1298: f64, t322: f64, t1089: f64, t175: f64, t384: f64, t1426: f64, t4818: f64, t360: f64, t368: f64, t1032: f64, t1423: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5243 = 0.85748036236139473944e-3_f64 * t1008 * t1576;
    let t5249 = t1298 * t322;
    let t5251 = t1089 * t175 * t5249;
    let t5253 = 0.17149607247227894789e-2_f64 * t384 * t5251;
    let t5255 = t1426 * t175 * t4818;
    let t5258 = t1298 * t360;
    let t5260 = t1089 * t368 * t5258;
    let t5263 = t1032 * t1423;
    (t5243, t5249, t5251, t5253, t5255, t5258, t5260, t5263)
}
