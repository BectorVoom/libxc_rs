//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 634/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk634<F: Float>(t3061: F, t5202: F, t1067: F, t1086: F, t1472: F, t1484: F, t2935: F, t2974: F, t3035: F, t3059: F, t402: F, t4087: F, t4182: F, t5117: F, t5123: F, t5155: F, t5158: F, t5167: F, t5169: F, t5173: F, t5189: F, t5192: F, t5198: F, t5203: F, t5219: F) -> (F, F) {
    let t5222 = t5202 * t3061;
    let t5225 = -F::new(0.3109e-1) * t5117 * t402 + F::new(2.0) * t4087 * t1472 - F::new(2.0) * t2935 * t5123 + F::new(1.0) * t1067 * t5155 + F::cast_from(0.32164683177870697974e2_f64) * t2974 * t5158 + t5167 - t5169 + t5173 - t5189 - t5192 - F::cast_from(0.19751789702565206229e-1_f64) * t5198 + F::cast_from(0.11696446794910408142e1_f64) * t4182 * t1484 - F::cast_from(0.11696446794910408142e1_f64) * t3035 * t5203 + F::cast_from(0.58482233974552040708e0_f64) * t1086 * t5219 + F::cast_from(0.17315755899375863299e2_f64) * t3059 * t5222;
    (t5222, t5225)
}
