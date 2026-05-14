//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1340/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1340<F: Float>(t2873: F, t3746: F, t4077: F, t11780: F, t4083: F, t1801: F, t8020: F, t4558: F, t2893: F, t4541: F, t3799: F, t9796: F, t13638: F, t4619: F, t12017: F, t2901: F) -> (F, F, F, F, F, F, F) {
    let t32734 = t3746 * t4077 * t2873;
    let t32742 = t11780 * t4083 * t2873;
    let t32746 = t1801 * t8020;
    let t32747 = t32746 * t4558;
    let t32750 = t2893 * t4541;
    let t32757 = t3799 * t9796;
    let t32760 = t13638 * t4619;
    let t32767 = t12017 * t2901;
    (t32734, t32742, t32747, t32750, t32757, t32760, t32767)
}
