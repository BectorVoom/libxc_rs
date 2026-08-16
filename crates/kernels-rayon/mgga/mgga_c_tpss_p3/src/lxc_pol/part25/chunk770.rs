//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 770/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk770(t5222: f64, t5269: f64, t1586: f64, t3118: f64, t466: f64, t5248: f64, t1561: f64, t1578: f64, t5242: f64, t1141: f64, t1143: f64, t220: f64, t3124: f64, t3126: f64, t3138: f64, t3139: f64, t468: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5270 = t5222 + t5269;
    let t5271 = param_beta * t5270;
    let t5275 = t1586 * t1586;
    let t5276 = t3118 * t5275;
    let t5279 = t466 * t5248;
    let t5283 = t1578 * t1561;
    let t5287 = t466 * t5242;
    let t5294 = 2.0_f64 * t1141 * t1143 * t5283 + t1141 * t1143 * t5287 + t220 * t468 * t5270 + 2.0_f64 * t3124 * t3126 * t5279 - t3138 * t3139 * t5279;
    (t5270, t5271, t5275, t5276, t5279, t5283, t5287, t5294)
}
