//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 634/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk634(t3061: f64, t5202: f64, t1067: f64, t1086: f64, t1472: f64, t1484: f64, t2935: f64, t2974: f64, t3035: f64, t3059: f64, t402: f64, t4087: f64, t4182: f64, t5117: f64, t5123: f64, t5155: f64, t5158: f64, t5167: f64, t5169: f64, t5173: f64, t5189: f64, t5192: f64, t5198: f64, t5203: f64, t5219: f64) -> (f64, f64) {
    let t5222 = t5202 * t3061;
    let t5225 = -0.3109e-1_f64 * t5117 * t402 + 2.0_f64 * t4087 * t1472 - 2.0_f64 * t2935 * t5123 + 1.0_f64 * t1067 * t5155 + 0.32164683177870697974e2_f64 * t2974 * t5158 + t5167 - t5169 + t5173 - t5189 - t5192 - 0.19751789702565206229e-1_f64 * t5198 + 0.11696446794910408142e1_f64 * t4182 * t1484 - 0.11696446794910408142e1_f64 * t3035 * t5203 + 0.58482233974552040708e0_f64 * t1086 * t5219 + 0.17315755899375863299e2_f64 * t3059 * t5222;
    (t5222, t5225)
}
