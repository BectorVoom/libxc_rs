//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2655/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2655(t12240: f64, t12379: f64, t12392: f64, t12409: f64, t16242: f64, t16261: f64, t16394: f64, t16401: f64, t3803: f64, t40000: f64, t40168: f64, t40169: f64, t5235: f64, t5246: f64, t5248: f64, t5249: f64, t54114: f64, t54116: f64, t54118: f64, t54125: f64, t54132: f64, t54133: f64, t54135: f64) -> f64 {
    let t54137 = t5246 * t5248 * t16242 * t12240 / 512.0_f64 + t16401 * t16261 / 512.0_f64 + t5246 * t5248 * t5249 * t40000 / 1536.0_f64 + t16394 * t12409 / 256.0_f64 + 7.0_f64 / 768.0_f64 * t54114 - 7.0_f64 / 384.0_f64 * t54116 - 7.0_f64 / 384.0_f64 * t54118 + 5.0_f64 / 128.0_f64 * t3803 * t40168 * t5249 * t40169 + 7.0_f64 / 768.0_f64 * t54125 - t5235 * t12392 / 3072.0_f64 - t5235 * t12379 / 3072.0_f64 + t54132 - 35.0_f64 / 192.0_f64 * t54133 - 35.0_f64 / 192.0_f64 * t54135;
    t54137
}
