//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1177/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1177<F: Float>(t7001: F, t974: F, t2530: F, t2559: F, t21503: F, t378: F, t2569: F, t2598: F, t6992: F, t993: F, t21601: F, t2593: F, t6996: F) -> (F, F, F, F, F, F, F) {
    let t21679 = t974 * t7001;
    let t21700 = t2530 * t2559;
    let t21715 = t378 * t21503;
    let t21721 = t2569 * t2598;
    let t21726 = t993 * t6992;
    let t21729 = t378 * t21601;
    let t21770 = t2593 * t6996;
    (t21679, t21700, t21715, t21721, t21726, t21729, t21770)
}
