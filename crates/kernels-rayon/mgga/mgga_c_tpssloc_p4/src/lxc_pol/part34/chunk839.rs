//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 839/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk839(t1363: f64, t16317: f64, t16394: f64, t19853: f64, t19879: f64, t20450: f64, t20454: f64, t20460: f64, t20465: f64, t20470: f64, t20475: f64, t20479: f64, t3803: f64, t5246: f64, t6396: f64) -> f64 {
    let t20484 = 7.0_f64 / 768.0_f64 * t19853 - 5.0_f64 / 256.0_f64 * t3803 * t20450 + t3803 * t20454 / 256.0_f64 + t16394 * t6396 / 128.0_f64 + t3803 * t20460 / 256.0_f64 + t3803 * t20465 / 256.0_f64 - t5246 * t20470 / 128.0_f64 + t5246 * t20475 / 512.0_f64 - t1363 * t20479 / 768.0_f64 - 7.0_f64 / 192.0_f64 * t19879 - 119.0_f64 / 1152.0_f64 * t16317;
    t20484
}
