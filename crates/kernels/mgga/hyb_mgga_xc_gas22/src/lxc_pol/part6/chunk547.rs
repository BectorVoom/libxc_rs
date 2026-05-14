//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 547/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk547<F: Float>(t1006: F, t2576: F, t2578: F, t2593: F, t997: F, t2598: F, t2601: F, t1014: F, t1016: F, t2472: F, t2475: F, t2482: F, t2518: F, t2526: F, t2570: F, t260: F, t2605: F, t2609: F) -> (F, F, F, F) {
    let t2613 = t2576 * t2578 * t1006;
    let t2617 = t997 * t2593 * t1006;
    let t2620 = t2598 * t2578;
    let t2621 = t2620 * t2601;
    let t2624 = -t2472 + t2475 - t2482 + t2518 + t2526 + t260 * t2605 + 0.19751673498613801407e-1 * t260 * t2570 - 0.11696447245269292414e1 * t2609 * t1016 + 0.11696447245269292414e1 * t1014 * t2613 - 0.5848223622634646207e0 * t1014 * t2617 - 0.17315859105681463759e2 * t1014 * t2621;
    (t2613, t2617, t2621, t2624)
}
