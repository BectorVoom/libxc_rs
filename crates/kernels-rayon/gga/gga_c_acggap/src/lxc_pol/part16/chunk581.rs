//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 581/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk581(t1137: f64, t1319: f64, t1008: f64, t1446: f64, t1451: f64, t3228: f64, t542: f64, t1588: f64, t537: f64, t1576: f64, t1298: f64, t322: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5175 = 7.0_f64 / 72.0_f64 * t1137 * t1319;
    let t5222 = 0.34299214494455789578e-2_f64 * t1008 * t1446;
    let t5224 = 0.17149607247227894789e-2_f64 * t1008 * t1451;
    let t5226 = t3228 * t542;
    let t5229 = 0.85748036236139473944e-3_f64 * t1008 * t1588;
    let t5240 = t3228 * t537;
    let t5243 = 0.85748036236139473944e-3_f64 * t1008 * t1576;
    let t5249 = t1298 * t322;
    (t5175, t5222, t5224, t5226, t5229, t5240, t5243, t5249)
}
