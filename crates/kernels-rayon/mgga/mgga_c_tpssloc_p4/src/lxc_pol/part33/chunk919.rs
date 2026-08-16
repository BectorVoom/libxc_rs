//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 919/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk919(t15875: f64, t15877: f64, t15890: f64, t15895: f64, t19591: f64, t11982: f64, t11984: f64, t1799: f64, t193: f64, t20077: f64, t20354: f64, t20355: f64, t20356: f64, t3918: f64, t5160: f64, t5161: f64, t571: f64, t6463: f64, t9457: f64, t9476: f64, t9484: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20360 = 24.0_f64 * t15875;
    let t20361 = 24.0_f64 * t15877;
    let t20365 = 0.51947577317044391276e2_f64 * t15890;
    let t20366 = 0.17544670867903938621e1_f64 * t15895;
    let t20370 = 12.0_f64 * t19591;
    let t20371 = -9.0_f64 * t1799 * t20077 * t3918 + 6.0_f64 * t193 * t20356 * t571 - 3.0_f64 * t5160 * t5161 * t6463 + t11982 - t11984 - t20354 + t20355 - t20360 - t20361 - t20365 - t20366 - t20370 - t9457 + t9476 + t9484;
    (t20360, t20361, t20365, t20366, t20370, t20371)
}
