//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1152/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1152<F: Float>(t222: F, t2753: F, t7578: F, t7637: F, t7646: F, t221: F, t23616: F, t450: F, t2633: F, t1036: F, t7607: F, t1037: F, t2632: F, t7577: F, t1063: F, t1071: F, t1078: F, t1086: F, t1846: F, t23583: F, t23832: F, t23835: F, t23839: F, t23843: F, t23847: F, t23851: F, t2693: F, t2729: F, t2732: F, t3038: F, t566: F, t7531: F, t7565: F, t7566: F, t7619: F, t7621: F) -> (F, F, F, F, F, F, F) {
    let t23854 = 0.71233333333333333332e-1 * t222 * t2753 * t7578;
    let t23857 = 0.4274e0 * t222 * t7646 * t7637;
    let t23863 = 0.11483599538271604938e-1 * t221 * t23616 * t450;
    let t23864 = t2633 * t2633;
    let t23867 = 24.0 * t7607 * t23864 * t1036;
    let t23870 = 8.0 * t2632 * t1037 * t7577;
    let t23871 = 0.13218100589565368422e2 * t222 * t566 * t7619 * t7621 - 0.27397333333333333333e0 * t222 * t1846 * t2729 * t2732 - 0.67471172535210825684e-1 * t222 * t3038 * t1078 * t1086 - 0.21309037037037037036e0 * t222 * t3038 * t1063 * t1071 - 0.38025319932552508021e2 * t222 * t566 * t7531 * t7566 + t23832 - t23835 - t23839 - t23843 + t23847 + t23851 + t23854 + t23857 + 0.6233709278045326953e3 * t7565 * t23583 * t2693 + t23863 + t23867 + t23870;
    (t23854, t23857, t23863, t23864, t23867, t23870, t23871)
}
