//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2654/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2654(t16398: f64, t20475: f64, t19731: f64, t3792: f64, t12429: f64, t16242: f64, t16394: f64, t16401: f64, t19631: f64, t19871: f64, t19956: f64, t19958: f64, t19989: f64, t20460: f64, t20463: f64, t20465: f64, t20470: f64, t20473: f64, t3803: f64, t3805: f64, t5187: f64, t5246: f64, t5248: f64, t5249: f64, t5250: f64, t550: f64, t56817: f64, t6394: f64, t74120: f64) -> (f64, f64) {
    let t74147 = t16398 * t20475;
    let t74174 = t3792 * t19731;
    let t74181 = 7.0_f64 / 1536.0_f64 * t5246 * t5248 * t74120 * t5250 + t3803 * t3805 * t16242 * t20463 / 256.0_f64 + t3803 * t3805 * t5249 * t550 * t19631 / 256.0_f64 - 7.0_f64 / 768.0_f64 * t74147 + t12429 * t20460 / 256.0_f64 + t3803 * t3805 * t56817 * t6394 / 256.0_f64 + t3803 * t3805 * t19956 * t19989 / 256.0_f64 + t12429 * t20465 / 256.0_f64 - t16401 * t20470 / 128.0_f64 - t5246 * t3805 * t19871 * t3792 * t5187 / 128.0_f64 + t16401 * t20475 / 512.0_f64 + t5246 * t5248 * t16242 * t20473 / 512.0_f64 + t5246 * t5248 * t5249 * t74174 / 512.0_f64 + t16394 * t19958 / 256.0_f64;
    (t74174, t74181)
}
