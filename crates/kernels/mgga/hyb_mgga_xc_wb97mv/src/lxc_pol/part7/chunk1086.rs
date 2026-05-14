//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1086/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1086<F: Float>(t3499: F, t9449: F, t11437: F, t11440: F, t11443: F, t11447: F, t11450: F, t11454: F, t11461: F, t11464: F, t11467: F, t11470: F, t2534: F, t2556: F, t2573: F, t2595: F, t3549: F, t3568: F, t7259: F, t7316: F, t7333: F, t9501: F, t9508: F) -> (F, F) {
    let t11474 = 4.0 * t9449 * t3499;
    let t11475 = -0.23392894490538584828e1 * t2573 * t11437 - 0.10389515463408878255e3 * t7259 * t11440 - 0.11696447245269292414e1 * t2573 * t11443 + 0.17315859105681463759e2 * t2595 * t11447 + 0.34631718211362927518e2 * t2595 * t11450 + 0.10254018858216406658e4 * t7316 * t11454 - 4.0 * t9508 * t3549 + 0.64327917994770140268e2 * t9501 * t3568 + 6.0 * t2556 * t11461 - 4.0 * t2534 * t11464 - 0.19298375398431042081e3 * t7333 * t11467 - 2.0 * t2534 * t11470 + t11474;
    (t11474, t11475)
}
