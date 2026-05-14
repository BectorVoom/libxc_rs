//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1162/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1162<F: Float>(t2840: F, t2856: F, t510: F, t534: F, t532: F, t516: F, t2847: F, t512: F, t535: F, t526: F, t19: F, t549: F, t8168: F, t8430: F, t554: F, t8188: F, t8473: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24636 = t2840 * t2856;
    let t24639 = t534 * t510;
    let t24640 = t24639 * t532;
    let t24649 = t516 * t532;
    let t24661 = 1.0 / t2847 / t512;
    let t24662 = t535 * t24661;
    let t24761 = t526 * t510;
    let t24785 = t19 * t549 * t8168;
    let t24802 = t19 * t549 * t8430;
    let t24824 = t554 * t8473 * t8188;
    (t24636, t24640, t24649, t24661, t24662, t24761, t24785, t24802, t24824)
}
