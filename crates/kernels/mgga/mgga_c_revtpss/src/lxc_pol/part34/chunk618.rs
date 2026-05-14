//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 618/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk618<F: Float>(t300: F, t6541: F, t6514: F, t1765: F, t5192: F, t1188: F, t3495: F, t6518: F, t1196: F, t1179: F, t6534: F, t3520: F, t3523: F, t3546: F, t5044: F, t6423: F, t6427: F, t6431: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6542 = t300 * t6541;
    let t6544 = 0.19751673498613801407e-1 * t300 * t6514;
    let t6546 = 0.11696447245269292414e1 * t5192 * t1765;
    let t6548 = t3495 * t6518 * t1188;
    let t6550 = 0.11696447245269292414e1 * t1196 * t6548;
    let t6552 = t1179 * t6534 * t1188;
    let t6554 = 0.5848223622634646207e0 * t1196 * t6552;
    let t6555 = t3520 * t6518;
    let t6556 = t6555 * t3523;
    let t6558 = 0.17315859105681463759e2 * t1196 * t6556;
    let t6563 = t3546 - 0.55555555555555555556e-2 * t5044 - 0.55555555555555555555e-2 * t6423 + 0.16666666666666666667e-1 * t6427 + 0.83333333333333333333e-2 * t6431;
    (t6542, t6544, t6546, t6548, t6550, t6552, t6554, t6556, t6558, t6563)
}
