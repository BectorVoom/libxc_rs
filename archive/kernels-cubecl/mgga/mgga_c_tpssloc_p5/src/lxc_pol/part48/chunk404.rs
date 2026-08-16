//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 404/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk404<F: Float>(t2718: F, t2719: F, t252: F, t2627: F, t2633: F, t814: F, t852: F, t829: F, t2679: F, t860: F, t2684: F, t235: F, t2710: F) -> (F, F, F, F, F, F) {
    let t2720 = t2718 * t2719;
    let t2728 = t2627 * t252;
    let t2729 = t2728 * t2633;
    let t2732 = t814 * t852;
    let t2733 = t2732 * t829;
    let t2736 = t860 * t2679;
    let t2738 = t860 * t2684;
    let t2740 = t235 * t2710;
    (t2720, t2729, t2733, t2736, t2738, t2740)
}
