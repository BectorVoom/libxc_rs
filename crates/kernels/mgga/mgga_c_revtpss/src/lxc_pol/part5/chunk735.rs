//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 735/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk735<F: Float>(t1198: F, t5192: F, t1765: F, t3531: F, t1756: F, t3495: F, t1189: F, t1196: F, t1179: F, t1188: F, t5180: F, t3520: F, t1187: F, t3523: F, t3358: F, t3546: F, t5044: F, t5049: F, t5054: F, t5058: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5194 = 0.5848223622634646207e0 * t5192 * t1198;
    let t5196 = 0.5848223622634646207e0 * t3531 * t1765;
    let t5197 = t3495 * t1756;
    let t5198 = t5197 * t1189;
    let t5200 = 0.11696447245269292414e1 * t1196 * t5198;
    let t5202 = t1179 * t5180 * t1188;
    let t5204 = 0.5848223622634646207e0 * t1196 * t5202;
    let t5205 = t3520 * t1756;
    let t5206 = t3523 * t1187;
    let t5207 = t5205 * t5206;
    let t5209 = 0.17315859105681463759e2 * t1196 * t5207;
    let t5215 = t3546 - 0.27777777777777777778e-2 * t3358 - 0.27777777777777777778e-2 * t5044 - 0.55555555555555555555e-2 * t5049 + 0.16666666666666666667e-1 * t5054 + 0.83333333333333333333e-2 * t5058;
    (t5194, t5196, t5197, t5198, t5200, t5202, t5204, t5205, t5206, t5207, t5209, t5215)
}
