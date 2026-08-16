//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1708/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1708(t12215: f64, t12335: f64, t12340: f64, t12346: f64, t12356: f64, t12358: f64, t12366: f64, t12386: f64, t12388: f64, t12395: f64, t12429: f64, t16366: f64, t16370: f64, t16379: f64, t16383: f64, t16387: f64, t16391: f64, t16394: f64, t16400: f64, t16401: f64, t16405: f64, t3803: f64, t3809: f64, t5246: f64, t5252: f64, t5303: f64) -> f64 {
    let t16411 = -t12335 + t12429 * t5303 / 384.0_f64 + t3803 * t16366 / 384.0_f64 + t3803 * t16370 / 768.0_f64 + 7.0_f64 / 576.0_f64 * t12340 - 119.0_f64 / 1728.0_f64 * t12346 - 35.0_f64 / 1152.0_f64 * t12356 + 7.0_f64 / 1152.0_f64 * t12358 - 119.0_f64 / 6912.0_f64 * t12366 - t12215 * t16379 / 4.0_f64 + t3803 * t16383 / 768.0_f64 + t5246 * t16387 / 512.0_f64 - t5246 * t16391 / 384.0_f64 + t16394 * t3809 / 384.0_f64 - t16400 + t16401 * t5252 / 768.0_f64 - 5.0_f64 / 768.0_f64 * t3803 * t16405 - 7.0_f64 / 2304.0_f64 * t12386 + 7.0_f64 / 4608.0_f64 * t12388 + 7.0_f64 / 4608.0_f64 * t12395;
    t16411
}
