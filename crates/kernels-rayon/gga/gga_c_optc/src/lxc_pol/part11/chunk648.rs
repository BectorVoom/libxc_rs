//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 648/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk648(t241: f64, t5198: f64, t3058: f64, t5202: f64, t3061: f64, t1102: f64, t5167: f64, t5169: f64, t5173: f64, t5189: f64, t5192: f64, t5226: f64, t5261: f64, t5266: f64, t5270: f64) -> (f64, f64, f64, f64, f64) {
    let t5306 = 0.19751789702565206229e-1_f64 * t241 * t5198;
    let t5307 = t3058 * t5202;
    let t5308 = t5307 * t3061;
    let t5310 = 0.17315755899375863299e2_f64 * t1102 * t5308;
    let t5311 = -t5167 + t5169 - t5173 + t5189 + t5192 + t5226 + t5306 - t5261 + t5266 - t5270 - t5310;
    (t5306, t5307, t5308, t5310, t5311)
}
