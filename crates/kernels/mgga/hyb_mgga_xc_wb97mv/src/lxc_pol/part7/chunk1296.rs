//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1296/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1296<F: Float>(t3596: F, t994: F, t4372: F, t7315: F, t3530: F, t2474: F, t957: F, t2473: F, t4287: F, t2476: F, t1003: F, t11376: F, t11564: F, t11567: F, t1428: F, t2572: F, t2605: F, t27242: F, t3608: F, t3613: F, t4386: F, t7231: F, t7434: F, t9261: F, t9311: F, t9387: F, t9465: F, t995: F) -> (F, F, F, F, F, F) {
    let t31726 = t3596 * t994;
    let t31730 = t3596 * t3596;
    let t31738 = t7315 * t4372;
    let t31750 = t3530 * t3530;
    let t31753 = 4.0 * t2474 * t31750 * t957;
    let t31754 = t4287 * t2473;
    let t31756 = 2.0 * t31754 * t2476;
    let t31757 = 0.11696447245269292414e1 * t7434 * t4386 + 0.23392894490538584828e1 * t1003 * t3613 * t9465 - 0.14035736694323150897e2 * t27242 * t1428 * t31726 + 0.23392894490538584828e1 * t1003 * t2572 * t31730 * t995 - 0.17315859105681463759e2 * t1003 * t11567 * t7231 - 0.10254018858216406658e4 * t1003 * t31738 * t9261 + 0.20779030926817756511e3 * t2605 * t11564 - 0.11696447245269292414e1 * t3608 * t9387 - 0.11696447245269292414e1 * t2605 * t11376 + 0.46785788981077169656e1 * t3608 * t9311 - t31753 - t31756;
    (t31726, t31730, t31750, t31753, t31756, t31757)
}
