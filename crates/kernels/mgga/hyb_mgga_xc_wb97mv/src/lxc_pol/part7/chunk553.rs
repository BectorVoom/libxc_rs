//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 553/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk553<F: Float>(t2468: F, t2471: F, t2478: F, t2514: F, t2522: F, t2526: F, t2529: F, t2534: F, t2536: F, t2551: F, t2556: F, t2559: F, t2566: F, t2568: F, t2573: F, t2575: F, t2590: F, t2595: F, t2598: F, t372: F, t968: F, t977: F, t987: F, t996: F) -> (F,) {
    let t2601 = -0.310907e-1 * t2526 * t372 + 2.0 * t2529 * t977 - 2.0 * t2534 * t2536 + 1.0 * t968 * t2551 + 0.32163958997385070134e2 * t2556 * t2559 + t2468 - t2471 + t2478 - t2514 - t2522 - 0.19751673498613801407e-1 * t2566 + 0.11696447245269292414e1 * t2568 * t996 - 0.11696447245269292414e1 * t2573 * t2575 + 0.5848223622634646207e0 * t987 * t2590 + 0.17315859105681463759e2 * t2595 * t2598;
    (t2601,)
}
