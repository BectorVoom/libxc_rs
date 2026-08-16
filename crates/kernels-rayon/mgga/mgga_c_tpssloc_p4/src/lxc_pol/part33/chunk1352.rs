//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1352/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1352(t1920: f64, t21122: f64, t21126: f64, t21520: f64, t21562: f64, t21574: f64, t23419: f64, t28558: f64, t2987: f64, t4509: f64, t6717: f64, t7574: f64, t88645: f64, t99774: f64, t99779: f64, t99785: f64, t99789: f64) -> f64 {
    let t106328 = -0.30279567070605293142e-3_f64 * t7574 * t28558 + t23419 * t21574 / 768.0_f64 + t6717 * t21562 / 48.0_f64 - t88645 / 2304.0_f64 - 0.30279567070605293142e-3_f64 * t99774 + t1920 * t4509 * t21122 / 72.0_f64 - t23419 * t21520 / 384.0_f64 + 0.30279567070605293142e-3_f64 * t99779 + t99785 / 288.0_f64 + t99789 / 216.0_f64 - t1920 * t2987 * t21126 / 48.0_f64;
    t106328
}
