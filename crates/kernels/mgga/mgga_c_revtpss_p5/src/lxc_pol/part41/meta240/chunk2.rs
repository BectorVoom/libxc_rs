//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 922/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk922<F: Float>(t1188: F, t6534: F, t3523: F, t6518: F, t1161: F, t1180: F, t1745: F, t1757: F, t3452: F, t3477: F, t3496: F, t3521: F, t435: F, t5120: F, t5158: F, t6435: F, t6437: F, t6441: F, t6473: F, t6476: F, t6481: F, t6487: F, t6503: F, t6506: F, t6514: F, t6519: F) -> (F, F, F) {
    let t6535 = t6534 * t1188;
    let t6538 = t6518 * t3523;
    let t6541 = -F::new(0.310907e-1) * t6481 * t435 + F::new(2.0) * t5120 * t1745 - F::new(2.0) * t3452 * t6487 + F::new(1.0) * t1161 * t6503 + F::cast_from(0.32163958997385070134e2_f64) * t3477 * t6506 + t6435 - t6437 + t6441 - t6473 - t6476 - F::cast_from(0.19751673498613801407e-1_f64) * t6514 + F::cast_from(0.11696447245269292414e1_f64) * t5158 * t1757 - F::cast_from(0.11696447245269292414e1_f64) * t3496 * t6519 + F::cast_from(0.5848223622634646207e0_f64) * t1180 * t6535 + F::cast_from(0.17315859105681463759e2_f64) * t3521 * t6538;
    (t6535, t6538, t6541)
}
