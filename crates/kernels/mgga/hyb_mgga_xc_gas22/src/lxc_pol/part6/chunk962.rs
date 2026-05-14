//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 962/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk962<F: Float>(t2594: F, t3596: F, t3604: F, t7165: F, t2598: F, t3579: F, t3605: F, t260: F, t3557: F, t1006: F, t9195: F, t997: F, t1014: F, t1016: F, t1442: F, t2609: F, t2613: F, t2621: F, t3591: F, t3597: F, t3601: F, t3606: F, t7222: F, t9103: F, t9106: F, t9108: F, t9110: F, t9170: F, t9260: F, t9268: F) -> (F, F, F, F, F, F) {
    let t9282 = t3596 * t2594;
    let t9285 = t3604 * t7165;
    let t9288 = t2598 * t3579;
    let t9289 = t9288 * t3605;
    let t9296 = t260 * t3557;
    let t9306 = t997 * t9195 * t1006;
    let t9309 = t9103 + t9106 + t9108 + t9110 + t9170 - t9260 + 0.11696447245269292414e1 * t3591 * t2613 + 0.11696447245269292414e1 * t1014 * t9282 - 0.17315859105681463759e2 * t1014 * t9285 - 0.34631718211362927518e2 * t1014 * t9289 + 0.23392894490538584828e1 * t2609 * t3597 - 0.34631718211362927518e2 * t2609 * t3606 - 0.11696447245269292414e1 * t9296 * t1016 - 0.11696447245269292414e1 * t2609 * t3601 - 0.17315859105681463759e2 * t3591 * t2621 - 0.5848223622634646207e0 * t7222 * t1442 - 0.5848223622634646207e0 * t1014 * t9306 - t9268;
    (t9282, t9285, t9289, t9296, t9306, t9309)
}
