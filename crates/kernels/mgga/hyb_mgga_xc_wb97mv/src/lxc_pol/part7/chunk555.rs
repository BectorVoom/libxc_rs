//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 555/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk555<F: Float>(t2572: F, t2574: F, t995: F, t2589: F, t986: F, t2594: F, t2597: F, t1003: F, t1005: F, t2468: F, t2471: F, t2478: F, t2514: F, t2522: F, t2566: F, t260: F, t2601: F, t2605: F) -> (F, F, F, F) {
    let t2609 = t2572 * t2574 * t995;
    let t2613 = t986 * t2589 * t995;
    let t2616 = t2594 * t2574;
    let t2617 = t2616 * t2597;
    let t2620 = -t2468 + t2471 - t2478 + t2514 + t2522 + t260 * t2601 + 0.19751673498613801407e-1 * t260 * t2566 - 0.11696447245269292414e1 * t2605 * t1005 + 0.11696447245269292414e1 * t1003 * t2609 - 0.5848223622634646207e0 * t1003 * t2613 - 0.17315859105681463759e2 * t1003 * t2617;
    (t2609, t2613, t2617, t2620)
}
