//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 334/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk334(t1072: f64, t1080: f64, t1081: f64, t1089: f64, t139: f64, t215: f64, t442: f64, t441: f64, t56: f64, t924: f64) -> (f64, f64, f64, f64, f64) {
    let t1091 = t1072 * t1080 * t1081;
    let t1093 = 0.5848223622634646207e0_f64 * t1089 * t1091;
    let t1095 = t215 * t139 * t442;
    let t1097 = t441 * t1095 / 288.0_f64;
    let t1098 = t56 * t924;
    (t1091, t1093, t1095, t1097, t1098)
}
