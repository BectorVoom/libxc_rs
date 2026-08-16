//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 666/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk666(t1091: f64, t4192: f64, t1551: f64, t3009: f64, t1542: f64, t2973: f64, t1082: f64, t1089: f64, t1072: f64, t1081: f64, t4180: f64, t2998: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4194 = 0.5848223622634646207e0_f64 * t4192 * t1091;
    let t4196 = 0.5848223622634646207e0_f64 * t3009 * t1551;
    let t4197 = t2973 * t1542;
    let t4198 = t4197 * t1082;
    let t4200 = 0.11696447245269292414e1_f64 * t1089 * t4198;
    let t4202 = t1072 * t4180 * t1081;
    let t4204 = 0.5848223622634646207e0_f64 * t1089 * t4202;
    let t4205 = t2998 * t1542;
    (t4194, t4196, t4197, t4198, t4200, t4202, t4204, t4205)
}
