//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1188/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1188<F: Float>(t1196: F, t16673: F, t3495: F, t5180: F, t1189: F, t3543: F, t5192: F, t3516: F, t5197: F, t12500: F, t5205: F, t1733: F, t3385: F, t3433: F, t3302: F, t5332: F) -> (F, F, F, F, F, F, F) {
    let t16675 = 0.34631718211362927518e2 * t1196 * t16673;
    let t16676 = t3495 * t5180;
    let t16677 = t16676 * t1189;
    let t16679 = 0.23392894490538584828e1 * t1196 * t16677;
    let t16681 = 0.17315859105681463759e2 * t5192 * t3543;
    let t16682 = t5197 * t3516;
    let t16684 = 0.11696447245269292414e1 * t1196 * t16682;
    let t16685 = t5205 * t12500;
    let t16687 = 0.17315859105681463759e2 * t1196 * t16685;
    let t16688 = t1733 * t3385;
    let t16690 = 6.0 * t3433 * t16688;
    let t16695 = t5332 * t3302;
    (t16675, t16679, t16681, t16684, t16687, t16690, t16695)
}
