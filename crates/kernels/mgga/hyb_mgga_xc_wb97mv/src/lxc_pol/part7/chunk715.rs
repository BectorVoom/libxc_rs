//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 715/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk715<F: Float>(t1114: F, t3718: F, t1111: F, t1117: F, t1148: F, t1528: F, t1544: F, t1547: F, t2817: F, t2823: F, t2828: F, t2860: F, t3680: F, t3685: F, t3689: F, t3697: F, t3704: F, t3708: F, t3712: F, t505: F, t511: F, t529: F) -> (F,) {
    let t3719 = t3718 * t1114;
    let t3722 = -0.33333333333333333333e-1 * t2817 * t3680 - 0.33333333333333333333e-1 * t2823 * t3680 + 0.384e-6 * t3685 * t3689 - 0.1e0 * t2828 * t3680 + 30.0 * t2860 * t1544 * t1111 - 36.0 * t1148 * t3697 * t1114 - 36.0 * t1148 * t1547 * t1111 + 42.0 * t529 * t3704 * t1114 + 2.0 * t505 * t3708 - 4.0 * t1117 * t3712 - 4.0 * t1117 * t1528 * t1111 + 6.0 * t511 * t3719;
    (t3722,)
}
