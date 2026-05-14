//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 976/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk976<F: Float>(t1408: F, t2555: F, t2533: F, t1420: F, t2572: F, t1427: F, t7318: F, t2574: F, t1416: F, t1428: F, t2529: F, t2536: F, t2551: F, t2559: F, t2568: F, t2575: F, t2598: F, t3544: F, t3564: F, t3597: F, t7249: F, t7316: F, t7322: F, t9458: F, t9465: F, t9493: F, t9496: F, t968: F, t977: F, t987: F) -> (F, F, F, F, F, F) {
    let t9501 = t1408 * t2555;
    let t9508 = t1408 * t2533;
    let t9511 = t1420 * t2572;
    let t9514 = t1427 * t7318;
    let t9515 = t9514 * t2574;
    let t9518 = 0.17315859105681463759e2 * t9458 * t2598 + 0.5848223622634646207e0 * t7249 * t1428 + 0.11696447245269292414e1 * t2568 * t3597 + 0.5848223622634646207e0 * t987 * t9465 + 1.0 * t968 * t9493 + 2.0 * t9496 * t977 + 1.0 * t3544 * t2551 + 0.32163958997385070134e2 * t9501 * t2559 + 1.0 * t7322 * t1416 + 2.0 * t2529 * t3564 - 2.0 * t9508 * t2536 - 0.11696447245269292414e1 * t9511 * t2575 + 0.10254018858216406658e4 * t7316 * t9515;
    (t9501, t9508, t9511, t9514, t9515, t9518)
}
