//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 879/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk879<F: Float>(t2627: F, t7737: F, t1014: F, t2704: F, t2626: F, t2685: F, t2694: F, t1101: F, t2647: F, t2697: F, t1085: F, t2689: F, t7532: F, t1099: F, t1090: F, t2715: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7738 = t7737 * t2627;
    let t7740 = t1014 * t2704;
    let t7742 = 0.32530743900905219526e-1 * t2626 * t7740;
    let t7743 = t1014 * t2685;
    let t7745 = 0.16265371950452609763e-1 * t2626 * t7743;
    let t7746 = t1014 * t2694;
    let t7748 = 0.48159733137676571078e0 * t2626 * t7746;
    let t7749 = t2647 * t1101;
    let t7751 = 0.21687162600603479684e-1 * t2626 * t7749;
    let t7752 = t2697 * t2704;
    let t7755 = t2689 * t7532 * t1085;
    let t7757 = 0.35089341735807877242e1 * t1099 * t7755;
    let t7758 = t2715 * t1090;
    (t7738, t7740, t7742, t7743, t7745, t7746, t7748, t7749, t7751, t7752, t7755, t7757, t7758)
}
