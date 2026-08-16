//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 765/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk765(t294: f64, t5184: f64, t5157: f64, t1551: f64, t4192: f64, t1081: f64, t2973: f64, t5161: f64, t1089: f64, t1072: f64, t5177: f64, t2998: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5185 = t294 * t5184;
    let t5187 = 0.19751673498613801407e-1_f64 * t294 * t5157;
    let t5189 = 0.11696447245269292414e1_f64 * t4192 * t1551;
    let t5191 = t2973 * t5161 * t1081;
    let t5193 = 0.11696447245269292414e1_f64 * t1089 * t5191;
    let t5195 = t1072 * t5177 * t1081;
    let t5197 = 0.5848223622634646207e0_f64 * t1089 * t5195;
    let t5198 = t2998 * t5161;
    (t5185, t5187, t5189, t5191, t5193, t5195, t5197, t5198)
}
