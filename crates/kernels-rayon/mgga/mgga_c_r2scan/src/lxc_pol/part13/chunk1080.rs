//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1080/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1080(t10868: f64, t6535: f64, t6536: f64, t2116: f64, t57: f64, t6257: f64, t261: f64, t3304: f64, t6457: f64, t10879: f64, t10891: f64, t3299: f64, t6470: f64) -> (f64, f64, f64, f64, f64) {
    let t38062 = t6535 * t10868 * t6536;
    let t38068 = t6257 * t57 * t2116;
    let t38069 = 0.98171973930797904389e-1_f64 * t38068;
    let t38074 = t3304 * t261 * t6457;
    let t38076 = t10879 * t10891;
    let t38079 = t3299 * t261 * t6470;
    (t38062, t38069, t38074, t38076, t38079)
}
