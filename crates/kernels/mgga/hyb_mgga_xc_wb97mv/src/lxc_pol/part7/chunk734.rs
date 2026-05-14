//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 734/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk734<F: Float>(t1128: F, t3813: F, t505: F, t513: F, t1117: F, t1122: F, t511: F, t1112: F, t1127: F, t1158: F, t1161: F, t1523: F, t2900: F, t2915: F, t2946: F, t2957: F, t3775: F, t3778: F, t3781: F, t3785: F, t3796: F, t3800: F, t3810: F) -> (F, F, F, F) {
    let t3814 = t1128 * t3813;
    let t3823 = t505 * t513;
    let t3826 = t1117 * t513;
    let t3829 = t511 * t1122;
    let t3840 = -t1112 * t1523 - 0.108e0 * t2915 * t3810 + 0.126e0 * t2957 * t3814 + 0.48e-1 * t1161 * t3775 - 0.48e-1 * t1158 * t3778 + 0.48e-1 * t1161 * t3781 - 50.0 / 9.0 * t3823 * t3785 - 100.0 / 9.0 * t3826 * t3785 - 100.0 / 9.0 * t3829 * t3785 - 0.12e-1 * t2900 * t3796 - 0.16e-1 * t1127 * t3800 - 0.12e-1 * t2900 * t3810 + 0.18e-1 * t2946 * t3814;
    (t3823, t3826, t3829, t3840)
}
