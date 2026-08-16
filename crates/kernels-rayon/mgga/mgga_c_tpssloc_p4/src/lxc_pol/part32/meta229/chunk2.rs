//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1050/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1050(t1156: f64, t6084: f64, t3403: f64, t6068: f64, t1129: f64, t1148: f64, t1683: f64, t1695: f64, t3332: f64, t3357: f64, t3376: f64, t3401: f64, t436: f64, t4797: f64, t4835: f64, t5985: f64, t5987: f64, t5991: f64, t6023: f64, t6026: f64, t6031: f64, t6037: f64, t6053: f64, t6056: f64, t6064: f64, t6069: f64) -> (f64, f64, f64) {
    let t6085 = t6084 * t1156;
    let t6088 = t6068 * t3403;
    let t6091 = -0.310907e-1_f64 * t6031 * t436 + 2.0_f64 * t4797 * t1683 - 2.0_f64 * t3332 * t6037 + 1.0_f64 * t1129 * t6053 + 0.32163958997385070134e2_f64 * t3357 * t6056 + t5985 - t5987 + t5991 - t6023 - t6026 - 0.19751673498613801407e-1_f64 * t6064 + 0.11696447245269292414e1_f64 * t4835 * t1695 - 0.11696447245269292414e1_f64 * t3376 * t6069 + 0.5848223622634646207e0_f64 * t1148 * t6085 + 0.17315859105681463759e2_f64 * t3401 * t6088;
    (t6085, t6088, t6091)
}
