//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 731/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk731<F: Float>(t1115: F, t1127: F, t1128: F, t1132: F, t1158: F, t1520: F, t1525: F, t2832: F, t2915: F, t2953: F, t3680: F, t3689: F, t3771: F, t3775: F, t3778: F, t3781: F, t3784: F, t3785: F, t3788: F, t3791: F, t3796: F, t3800: F, t3803: F) -> (F,) {
    let t3807 = -0.1e0 * t2832 * t3680 + 0.384e-6 * t3771 * t3689 + 0.16e-1 * t1132 * t3775 - 0.16e-1 * t1127 * t3778 + 0.16e-1 * t1132 * t3781 - 100.0 / 3.0 * t3784 * t3785 - 100.0 / 3.0 * t3788 * t3785 + 0.9e-1 * t2953 * t1128 * t3791 - 0.108e0 * t2915 * t3796 - 0.48e-1 * t1158 * t3800 + 2.0 * t3803 * t1525 - t1520 * t1115;
    (t3807,)
}
