//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1103/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1103<F: Float>(t11717: F, t3688: F, t1153: F, t4541: F, t4550: F, t2848: F, t4554: F, t1122: F, t1114: F, t4603: F, t4571: F, t10147: F, t10162: F, t10166: F, t10177: F, t10182: F, t10186: F, t10190: F, t10194: F, t1111: F, t1117: F, t1133: F, t1148: F, t11694: F, t11704: F, t11710: F, t2860: F, t3729: F, t4551: F, t4591: F, t4594: F, t4597: F, t505: F, t529: F, t7938: F, t9850: F) -> (F, F, F, F, F, F, F, F) {
    let t11718 = t11717 * t3688;
    let t11734 = t1153 * t4541;
    let t11741 = t1153 * t4550;
    let t11748 = t2848 * t4554;
    let t11752 = t1122 * t4550;
    let t11753 = t11752 * t1114;
    let t11756 = t4603 * t1114;
    let t11761 = t4571 * t1122;
    let t11764 = -0.53333333333333333333e-3 * t10194 * t11710 + 0.14e1 * t10162 * t11694 - 0.16e-2 * t10182 * t11710 - 0.1536e-5 * t10177 * t11718 - 0.53333333333333333333e-3 * t9850 * t11710 + 0.4608e-5 * t10186 * t11704 - 0.16e-2 * t10147 * t11710 - 0.4608e-5 * t10190 * t11718 + 0.13333333333333333333e0 * t10166 * t11694 + 120.0 * t7938 * t4591 * t1111 - 180.0 * t2860 * t11734 * t1114 + 30.0 * t2860 * t4594 * t1111 - 36.0 * t1148 * t11741 * t1114 - 36.0 * t1148 * t4597 * t1111 + 42.0 * t529 * t11748 * t1114 - 4.0 * t1117 * t11753 + 2.0 * t505 * t11756 + 0.6e-2 * t3729 * t4551 - 0.12e-1 * t11761 * t1133;
    (t11734, t11741, t11748, t11752, t11753, t11756, t11761, t11764)
}
