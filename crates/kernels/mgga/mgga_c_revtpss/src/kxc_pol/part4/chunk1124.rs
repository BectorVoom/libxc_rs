//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1124/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1124<F: Float>(t15396: F, t2875: F, t11385: F, t4644: F, t945: F, t11456: F, t15350: F, t15373: F, t15377: F, t15379: F, t15382: F, t15385: F, t15388: F, t15392: F, t15395: F, t1634: F, t2982: F, t3015: F, t311: F, t4708: F, t955: F) -> (F, F) {
    let t15397 = t15396 * t2875;
    let t15399 = 0.51726012919273400301e3 * t11385 * t15397;
    let t15400 = t4644 * t945;
    let t15403 = 0.17315859105681463759e2 * t15350 * t3015 + 0.5848223622634646207e0 * t11456 * t1634 + 0.11696447245269292414e1 * t2982 * t4708 - 0.310907e-1 * t15373 * t311 + t15377 - t15379 + t15382 + t15385 + t15388 - t15392 - t15395 - t15399 + 2.0 * t15400 * t955;
    (t15399, t15403)
}
