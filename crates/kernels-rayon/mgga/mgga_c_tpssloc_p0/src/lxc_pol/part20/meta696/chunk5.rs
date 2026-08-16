//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2659/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2659(t1824: f64, t3791: f64, t12168: f64, t12369: f64, t1352: f64, t16224: f64, t16305: f64, t16364: f64, t3803: f64, t3805: f64, t3851: f64, t40089: f64, t40114: f64, t40116: f64, t40124: f64, t40126: f64, t40128: f64, t40131: f64, t40139: f64, t40145: f64, t5246: f64, t5248: f64, t5249: f64, t53958: f64, t54068: f64) -> (f64, f64) {
    let t54258 = t1824 * t3791;
    let t54277 = -7.0_f64 / 16.0_f64 * t40089 + t3803 * t3805 * t16364 * t3851 / 256.0_f64 - t3803 * t5248 * t53958 * t1352 / 1024.0_f64 + 7.0_f64 / 1536.0_f64 * t40114 - 35.0_f64 / 384.0_f64 * t40116 - 3.0_f64 / 128.0_f64 * t5246 * t16305 * t54258 * t12369 + 5.0_f64 / 128.0_f64 * t5246 * t16224 * t54068 * t12369 + 595.0_f64 / 3456.0_f64 * t40124 - 119.0_f64 / 4608.0_f64 * t40126 + 7.0_f64 / 4608.0_f64 * t40128 - 7.0_f64 / 768.0_f64 * t40131 - t3803 * t5248 * t5249 * t12168 / 3072.0_f64 - 7.0_f64 / 192.0_f64 * t40139 - 595.0_f64 / 3456.0_f64 * t40145;
    (t54258, t54277)
}
