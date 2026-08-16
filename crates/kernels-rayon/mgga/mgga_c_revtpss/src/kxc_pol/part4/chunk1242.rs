//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1242/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1242(t11456: f64, t15350: f64, t15373: f64, t15377: f64, t15379: f64, t15382: f64, t15385: f64, t15388: f64, t15392: f64, t15395: f64, t15399: f64, t15400: f64, t1634: f64, t2982: f64, t3015: f64, t311: f64, t4708: f64, t955: f64) -> f64 {
    let t15403 = 0.17315859105681463759e2_f64 * t15350 * t3015 + 0.5848223622634646207e0_f64 * t11456 * t1634 + 0.11696447245269292414e1_f64 * t2982 * t4708 - 0.310907e-1_f64 * t15373 * t311 + t15377 - t15379 + t15382 + t15385 + t15388 - t15392 - t15395 - t15399 + 2.0_f64 * t15400 * t955;
    t15403
}
