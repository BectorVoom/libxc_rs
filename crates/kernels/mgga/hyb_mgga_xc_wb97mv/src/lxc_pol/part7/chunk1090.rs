//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1090/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1090<F: Float>(t260: F, t4354: F, t1003: F, t1005: F, t11253: F, t11255: F, t11325: F, t11327: F, t11333: F, t11338: F, t11340: F, t11342: F, t11343: F, t11348: F, t11376: F, t11394: F, t11416: F, t11435: F, t11475: F, t11537: F, t2605: F, t3608: F, t3618: F, t4390: F, t4394: F) -> (F, F) {
    let t11541 = t260 * t4354;
    let t11544 = -t11253 + t11255 + t11325 + t11327 + 0.19751673498613801407e-1 * t260 * t11333 + t11338 + t11340 + t11342 - 0.34631718211362927518e2 * t1003 * t11343 - 0.10254018858216406658e4 * t1003 * t11348 - 0.11696447245269292414e1 * t3608 * t3618 - 0.5848223622634646207e0 * t1003 * t11376 - 0.5848223622634646207e0 * t2605 * t4390 - 0.17315859105681463759e2 * t2605 * t4394 + t260 * (t11394 + t11435 + t11475 + t11537) - t11416 - 0.5848223622634646207e0 * t11541 * t1005;
    (t11541, t11544)
}
