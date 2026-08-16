//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1513/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1513(t12250: f64, t12419: f64, t16233: f64, t16305: f64, t16394: f64, t1799: f64, t19871: f64, t19956: f64, t20416: f64, t20448: f64, t20450: f64, t20454: f64, t20463: f64, t20465: f64, t3803: f64, t3805: f64, t5248: f64, t5249: f64, t550: f64, t56878: f64, t6394: f64, t6396: f64, t74110: f64, t74120: f64, t74147: f64, t74189: f64, t74415: f64, t75008: f64) -> f64 {
    let t80303 = -5.0_f64 / 64.0_f64 * t16394 * t20450 - 5.0_f64 / 128.0_f64 * t3803 * t12419 * t19871 * t20448 + t16394 * t20454 / 64.0_f64 - 7.0_f64 / 96.0_f64 * t74110 + t16233 * t3805 * t74120 * t12250 * t1799 / 32.0_f64 - 3.0_f64 / 256.0_f64 * t16233 * t5248 * t19871 * t75008 + t3803 * t3805 * t19956 * t20463 / 128.0_f64 + t3803 * t3805 * t5249 * t550 * t20416 / 192.0_f64 + t56878 * t6396 / 64.0_f64 + t16394 * t20465 / 64.0_f64 - 7.0_f64 / 192.0_f64 * t74147 + t3803 * t16305 * t74415 * t6394 / 64.0_f64 - 7.0_f64 / 96.0_f64 * t74189;
    t80303
}
