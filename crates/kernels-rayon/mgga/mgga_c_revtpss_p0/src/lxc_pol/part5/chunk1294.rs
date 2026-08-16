//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1294/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1294(t1169: f64, t20520: f64, t1179: f64, t6513: f64, t1188: f64, t20382: f64, t1160: f64, t6481: f64, t1161: f64, t1170: f64, t1180: f64, t1189: f64, t12423: f64, t12481: f64, t12491: f64, t17089: f64, t1757: f64, t20450: f64, t20452: f64, t3491: f64, t5158: f64, t5181: f64, t6506: f64, t6519: f64, t6535: f64, t6538: f64) -> f64 {
    let t20521 = t20520 * t1169;
    let t20526 = t6513 * t1179;
    let t20537 = t20382 * t1188;
    let t20542 = t6481 * t1160;
    let t20545 = 1.0_f64 * t1161 * t20521 + 0.32163958997385070134e2_f64 * t12423 * t6506 + 0.5848223622634646207e0_f64 * t20526 * t1189 + 0.11696447245269292414e1_f64 * t17089 * t1757 + 0.11696447245269292414e1_f64 * t5158 * t5181 - 0.11696447245269292414e1_f64 * t12491 * t6519 + 0.5848223622634646207e0_f64 * t3491 * t6535 + 0.5848223622634646207e0_f64 * t1180 * t20537 + 0.17315859105681463759e2_f64 * t12481 * t6538 - t20450 - t20452 + 1.0_f64 * t20542 * t1170;
    t20545
}
