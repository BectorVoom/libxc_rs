//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2653/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2653(t12283: f64, t16405: f64, t40167: f64, t820: f64, t1799: f64, t3791: f64, t40138: f64, t5259: f64, t16248: f64, t5293: f64, t16275: f64, t120: f64, t12178: f64, t12420: f64, t12429: f64, t1352: f64, t16018: f64, t16224: f64, t16225: f64, t16227: f64, t16364: f64, t16370: f64, t16387: f64, t16391: f64, t16401: f64, t3793: f64, t3803: f64, t3805: f64, t3807: f64, t5246: f64, t5248: f64, t5249: f64) -> (f64, f64) {
    let t54059 = t12283 * t16405;
    let t54063 = t40167 * t820;
    let t54068 = t1799 * t3791;
    let t54086 = t40138 * t5259;
    let t54088 = t12283 * t16248;
    let t54090 = t40138 * t5293;
    let t54092 = t12283 * t16275;
    let t54100 = 35.0_f64 / 384.0_f64 * t54059 - 5.0_f64 / 128.0_f64 * t12429 * t16227 + 15.0_f64 / 128.0_f64 * t3803 * t54063 * t16225 * t12420 - 5.0_f64 / 256.0_f64 * t3803 * t16224 * t54068 * t3807 + t12429 * t16370 / 256.0_f64 + t3803 * t3805 * t120 * t16018 * t1352 / 256.0_f64 - t16401 * t16391 / 128.0_f64 - t5246 * t3805 * t16364 * t3793 / 128.0_f64 - 7.0_f64 / 192.0_f64 * t54086 - 7.0_f64 / 384.0_f64 * t54088 + 7.0_f64 / 768.0_f64 * t54090 + 7.0_f64 / 1536.0_f64 * t54092 + 3.0_f64 / 512.0_f64 * t16401 * t16387 - t3803 * t5248 * t5249 * t12178 / 3072.0_f64;
    (t54068, t54100)
}
