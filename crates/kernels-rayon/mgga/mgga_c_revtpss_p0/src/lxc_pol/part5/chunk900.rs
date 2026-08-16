//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 900/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk900(t1188: f64, t6534: f64, t3523: f64, t6518: f64, t1161: f64, t1180: f64, t1745: f64, t1757: f64, t3452: f64, t3477: f64, t3496: f64, t3521: f64, t435: f64, t5120: f64, t5158: f64, t6435: f64, t6437: f64, t6441: f64, t6473: f64, t6476: f64, t6481: f64, t6487: f64, t6503: f64, t6506: f64, t6514: f64, t6519: f64) -> (f64, f64, f64) {
    let t6535 = t6534 * t1188;
    let t6538 = t6518 * t3523;
    let t6541 = -0.310907e-1_f64 * t6481 * t435 + 2.0_f64 * t5120 * t1745 - 2.0_f64 * t3452 * t6487 + 1.0_f64 * t1161 * t6503 + 0.32163958997385070134e2_f64 * t3477 * t6506 + t6435 - t6437 + t6441 - t6473 - t6476 - 0.19751673498613801407e-1_f64 * t6514 + 0.11696447245269292414e1_f64 * t5158 * t1757 - 0.11696447245269292414e1_f64 * t3496 * t6519 + 0.5848223622634646207e0_f64 * t1180 * t6535 + 0.17315859105681463759e2_f64 * t3521 * t6538;
    (t6535, t6538, t6541)
}
