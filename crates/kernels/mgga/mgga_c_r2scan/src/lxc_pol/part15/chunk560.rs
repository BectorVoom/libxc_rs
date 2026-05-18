//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 560/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk560<F: Float>(t2214: F, t923: F, t514: F, t1604: F, t2605: F, t788: F, t938: F, t2201: F, t785: F, t910: F, t2207: F, t780: F, t980: F) -> (F, F, F, F, F, F, F, F) {
    let t2682 = t2214 * t923;
    let t2683 = t514 * t2682;
    let t2685 = t1604 * t2605;
    let t2687 = t788 * t938;
    let t2689 = t2201 * t785 * t2687;
    let t2691 = t788 * t910;
    let t2693 = t2207 * t785 * t2691;
    let t2696 = t980 * t780;
    (t2682, t2683, t2685, t2687, t2689, t2691, t2693, t2696)
}
