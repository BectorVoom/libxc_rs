//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1318/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1318<F: Float>(t2533: F, t4327: F, t23180: F, t23183: F, t23522: F, t2536: F, t2573: F, t27021: F, t27024: F, t27027: F, t27582: F, t31730: F, t31753: F, t31756: F, t31763: F, t31767: F, t31769: F, t31779: F, t31782: F, t31810: F, t31914: F, t31916: F, t31919: F, t31929: F, t31933: F, t31936: F, t31938: F, t31940: F, t31944: F, t31946: F, t372: F, t9524: F, t995: F) -> (F,) {
    let t32182 = t4327 * t2533;
    let t32202 = t31753 + t31756 - 2.0 * t32182 * t2536 - t31763 - t31767 - t31769 - t31914 - t31916 - t31919 - t31933 - t31936 - t31938 - 0.23392894490538584828e1 * t2573 * t31730 * t995 + 24.0 * t27582 * t9524 + t31940 - t31944 - t31946 - 0.19751673498613801407e-1 * t31929 - 0.310907e-1 * (t23522 - 0.10654518518518518518e0 * t23180 + 0.22831111111111111111e-1 * t23183 - 0.10654518518518518518e0 * t27021 + 0.91324444444444444442e-1 * t27024 - 0.34246666666666666666e-1 * t27027 + 0.22831111111111111111e-1 * t31779 - 0.34246666666666666666e-1 * t31782 + 0.5137e-1 * t31810) * t372;
    (t32202,)
}
