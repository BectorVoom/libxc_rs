//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 945/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk945(t20473: f64, t5248: f64, t5249: f64, t1367: f64, t20416: f64, t820: f64, t1363: f64, t16317: f64, t16394: f64, t19853: f64, t19879: f64, t20450: f64, t20454: f64, t20460: f64, t20465: f64, t20470: f64, t3803: f64, t5246: f64, t6396: f64) -> (f64, f64, f64) {
    let t20475 = t5248 * t5249 * t20473;
    let t20479 = t1367 * t820 * t20416;
    let t20484 = 7.0_f64 / 768.0_f64 * t19853 - 5.0_f64 / 256.0_f64 * t3803 * t20450 + t3803 * t20454 / 256.0_f64 + t16394 * t6396 / 128.0_f64 + t3803 * t20460 / 256.0_f64 + t3803 * t20465 / 256.0_f64 - t5246 * t20470 / 128.0_f64 + t5246 * t20475 / 512.0_f64 - t1363 * t20479 / 768.0_f64 - 7.0_f64 / 192.0_f64 * t19879 - 119.0_f64 / 1152.0_f64 * t16317;
    (t20475, t20479, t20484)
}
