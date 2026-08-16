//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2690/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2690(t12283: f64, t16244: f64, t1307: f64, t3791: f64, t12279: f64, t12419: f64, t12420: f64, t12422: f64, t12426: f64, t12429: f64, t16233: f64, t16242: f64, t16305: f64, t16366: f64, t16394: f64, t19876: f64, t3793: f64, t3803: f64, t39975: f64, t40329: f64, t5246: f64, t5248: f64, t5249: f64, t5259: f64, t5303: f64, t54014: f64, t54739: f64, t54744: f64, t54745: f64, t54750: f64, t554: f64, t559: f64) -> f64 {
    let t54764 = t12283 * t16244;
    let t54770 = t3791 * t1307;
    let t54776 = 3.0_f64 / 512.0_f64 * t5246 * t5248 * t16242 * t3793 + t54739 * t554 * t559 / 3072.0_f64 + t54744 * t5248 * t5249 * t54745 / 128.0_f64 + 7.0_f64 / 192.0_f64 * t54750 - 5.0_f64 / 256.0_f64 * t3803 * t12419 * t16242 * t12420 + t39975 * t5259 / 256.0_f64 + t19876 * t12279 / 512.0_f64 + t16394 * t12426 / 256.0_f64 - 5.0_f64 / 256.0_f64 * t16394 * t12422 - 7.0_f64 / 192.0_f64 * t54764 + t39975 * t5303 / 256.0_f64 + t12429 * t16366 / 128.0_f64 + 3.0_f64 / 128.0_f64 * t16233 * t16305 * t54014 * t54770 - 7.0_f64 / 4608.0_f64 * t40329;
    t54776
}
