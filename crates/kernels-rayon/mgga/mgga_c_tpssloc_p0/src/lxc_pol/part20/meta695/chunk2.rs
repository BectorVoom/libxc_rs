//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2649/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2649(t12282: f64, t5234: f64, t3809: f64, t120: f64, t16205: f64, t12283: f64, t16227: f64, t1351: f64, t5286: f64, t12429: f64, t1352: f64, t16148: f64, t16224: f64, t16305: f64, t16308: f64, t16311: f64, t16314: f64, t16401: f64, t3803: f64, t3805: f64, t3807: f64, t39945: f64, t39948: f64, t39950: f64, t39956: f64, t39958: f64, t39960: f64, t40197: f64, t5246: f64) -> (f64, f64) {
    let t53945 = t5234 * t12282;
    let t53946 = t53945 * t3809;
    let t53958 = t120 * t16205;
    let t53965 = t12283 * t16227;
    let t53973 = t5286 * t1351;
    let t53978 = -7.0_f64 / 192.0_f64 * t53946 + 7.0_f64 / 768.0_f64 * t39945 - 119.0_f64 / 2304.0_f64 * t39948 - 119.0_f64 / 4608.0_f64 * t39950 + 7.0_f64 / 1536.0_f64 * t39956 - 7.0_f64 / 768.0_f64 * t39958 + 7.0_f64 / 1536.0_f64 * t39960 - t5246 * t16305 * t16311 * t40197 / 128.0_f64 + t3803 * t3805 * t53958 * t3807 / 256.0_f64 - t16401 * t16314 / 64.0_f64 + 35.0_f64 / 192.0_f64 * t53965 - 5.0_f64 / 128.0_f64 * t3803 * t16224 * t16148 * t1352 + t12429 * t16308 / 128.0_f64 + t3803 * t16305 * t53973 * t3807 / 128.0_f64;
    (t53958, t53978)
}
