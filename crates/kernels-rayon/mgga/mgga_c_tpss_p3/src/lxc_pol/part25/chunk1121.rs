//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1121/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1121(t1043: f64, t15408: f64, t1024: f64, t5117: f64, t9504: f64, t2998: f64, t5177: f64, t4206: f64, t1089: f64, t5161: f64, t9347: f64, t9172: f64) -> (f64, f64, f64, f64, f64) {
    let t15409 = t15408 * t1043;
    let t15411 = 1.0_f64 * t1024 * t15409;
    let t15413 = 0.16081979498692535067e2_f64 * t9504 * t5117;
    let t15414 = t2998 * t5177;
    let t15415 = t15414 * t4206;
    let t15417 = 0.17315859105681463759e2_f64 * t1089 * t15415;
    let t15418 = t9347 * t5161;
    let t15419 = t15418 * t4206;
    let t15421 = 0.10389515463408878255e3_f64 * t1089 * t15419;
    let t15422 = t9172 * t5161;
    (t15411, t15413, t15417, t15421, t15422)
}
