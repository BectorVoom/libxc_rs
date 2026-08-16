//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1294/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1294<F: Float>(t1169: F, t20520: F, t1179: F, t6513: F, t1188: F, t20382: F, t1160: F, t6481: F, t1161: F, t1170: F, t1180: F, t1189: F, t12423: F, t12481: F, t12491: F, t17089: F, t1757: F, t20450: F, t20452: F, t3491: F, t5158: F, t5181: F, t6506: F, t6519: F, t6535: F, t6538: F) -> F {
    let t20521 = t20520 * t1169;
    let t20526 = t6513 * t1179;
    let t20537 = t20382 * t1188;
    let t20542 = t6481 * t1160;
    let t20545 = F::cast_from(1.0_f64) * t1161 * t20521 + F::cast_from(0.32163958997385070134e2_f64) * t12423 * t6506 + F::cast_from(0.5848223622634646207e0_f64) * t20526 * t1189 + F::cast_from(0.11696447245269292414e1_f64) * t17089 * t1757 + F::cast_from(0.11696447245269292414e1_f64) * t5158 * t5181 - F::cast_from(0.11696447245269292414e1_f64) * t12491 * t6519 + F::cast_from(0.5848223622634646207e0_f64) * t3491 * t6535 + F::cast_from(0.5848223622634646207e0_f64) * t1180 * t20537 + F::cast_from(0.17315859105681463759e2_f64) * t12481 * t6538 - t20450 - t20452 + F::cast_from(1.0_f64) * t20542 * t1170;
    t20545
}
