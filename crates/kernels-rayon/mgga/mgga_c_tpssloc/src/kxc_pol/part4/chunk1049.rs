//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1049/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1049(t17349: f64, t932: f64, t5769: f64, t942: f64, t17297: f64, t951: f64, t13515: f64, t1557: f64, t4354: f64, t4396: f64, t10747: f64, t10765: f64, t10825: f64, t14332: f64, t1581: f64, t17197: f64, t2900: f64, t4449: f64, t4472: f64, t5762: f64, t5775: f64, t5791: f64, t5794: f64, t924: f64, t943: f64, t952: f64) -> (f64, f64, f64) {
    let t17350 = t17349 * t932;
    let t17355 = t5769 * t942;
    let t17366 = t17297 * t951;
    let t17372 = 2.0_f64 * t13515 * t1557;
    let t17374 = 2.0_f64 * t4354 * t4396;
    let t17375 = -t17197 + 1.0_f64 * t924 * t17350 + 0.32163958997385070134e2_f64 * t10765 * t5762 + 0.5848223622634646207e0_f64 * t17355 * t952 + 0.11696447245269292414e1_f64 * t14332 * t1581 + 0.11696447245269292414e1_f64 * t4449 * t4472 - 0.11696447245269292414e1_f64 * t10747 * t5775 + 0.5848223622634646207e0_f64 * t2900 * t5791 + 0.5848223622634646207e0_f64 * t943 * t17366 + 0.17315859105681463759e2_f64 * t10825 * t5794 - t17372 - t17374;
    (t17372, t17374, t17375)
}
