//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 540/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk540(t1156: f64, t4857: f64, t1694: f64, t3403: f64, t1155: f64, t1129: f64, t1138: f64, t1148: f64, t1157: f64, t1683: f64, t1695: f64, t3327: f64, t3332: f64, t3357: f64, t3371: f64, t3376: f64, t3401: f64, t436: f64, t4739: f64, t4742: f64, t4744: f64, t4747: f64, t4784: f64, t4788: f64, t4794: f64, t4797: f64, t4802: f64, t4820: f64, t4824: f64, t4833: f64, t4835: f64, t4840: f64) -> f64 {
    let t4858 = t4857 * t1156;
    let t4861 = t1694 * t3403;
    let t4862 = t4861 * t1155;
    let t4865 = -0.310907e-1_f64 * t4794 * t436 + 1.0_f64 * t4797 * t1138 + 1.0_f64 * t3327 * t1683 - 2.0_f64 * t3332 * t4802 + 1.0_f64 * t1129 * t4820 + 0.32163958997385070134e2_f64 * t3357 * t4824 + t4739 - t4742 - t4744 + t4747 - t4784 - t4788 - 0.19751673498613801407e-1_f64 * t4833 + 0.5848223622634646207e0_f64 * t4835 * t1157 + 0.5848223622634646207e0_f64 * t3371 * t1695 - 0.11696447245269292414e1_f64 * t3376 * t4840 + 0.5848223622634646207e0_f64 * t1148 * t4858 + 0.17315859105681463759e2_f64 * t3401 * t4862;
    t4865
}
