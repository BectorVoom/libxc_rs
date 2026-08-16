//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1349/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1349(t21390: f64, t3032: f64, t1618: f64, t21393: f64, t21398: f64, t21405: f64, t21603: f64, t25580: f64, t28578: f64, t360: f64, t5857: f64, t5861: f64, t5880: f64, t6765: f64, t82987: f64, t83054: f64, t83058: f64, t83065: f64, t83142: f64, t88342: f64, t88600: f64, t99509: f64, t99539: f64) -> (f64, f64) {
    let t106209 = t21390 * t3032;
    let t106218 = -t99509 / 768.0_f64 + t83054 * t21393 / 256.0_f64 - t83058 * t21398 / 256.0_f64 - t88600 * t5880 / 512.0_f64 + t83065 * t21405 / 1536.0_f64 + t6765 * t21603 / 2304.0_f64 + 5.0_f64 / 2304.0_f64 * t25580 * t5861 + 0.60559134141210586284e-3_f64 * t88342 * t28578 + 0.10093189023535097714e-3_f64 * t82987 * t83142 * t106209 * t360 + t25580 * t5857 / 768.0_f64 + t99539 * t1618 / 512.0_f64;
    (t106209, t106218)
}
