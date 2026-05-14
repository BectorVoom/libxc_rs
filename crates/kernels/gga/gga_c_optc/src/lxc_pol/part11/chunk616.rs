//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 616/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk616<F: Float>(t3061: F, t5202: F, t1067: F, t1086: F, t1472: F, t1484: F, t2935: F, t2974: F, t3035: F, t3059: F, t402: F, t4087: F, t4182: F, t5117: F, t5123: F, t5155: F, t5158: F, t5167: F, t5169: F, t5173: F, t5189: F, t5192: F, t5198: F, t5203: F, t5219: F) -> (F, F) {
    let t5222 = t5202 * t3061;
    let t5225 = -0.3109e-1 * t5117 * t402 + 2.0 * t4087 * t1472 - 2.0 * t2935 * t5123 + 1.0 * t1067 * t5155 + 0.32164683177870697974e2 * t2974 * t5158 + t5167 - t5169 + t5173 - t5189 - t5192 - 0.19751789702565206229e-1 * t5198 + 0.11696446794910408142e1 * t4182 * t1484 - 0.11696446794910408142e1 * t3035 * t5203 + 0.58482233974552040708e0 * t1086 * t5219 + 0.17315755899375863299e2 * t3059 * t5222;
    (t5222, t5225)
}
