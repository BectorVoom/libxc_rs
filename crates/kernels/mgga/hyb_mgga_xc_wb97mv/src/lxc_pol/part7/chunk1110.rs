//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1110/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1110<F: Float>(t1137: F, t11826: F, t3704: F, t529: F, t1528: F, t505: F, t3678: F, t4951: F, t1117: F, t3711: F, t3718: F, t511: F, t1544: F, t2860: F, t1148: F, t3697: F) -> (F, F, F, F, F, F, F, F) {
    let t11827 = t11826 * t1137;
    let t11834 = t529 * t3704;
    let t11837 = t505 * t1528;
    let t11840 = t3678 * t4951;
    let t11843 = t1117 * t3711;
    let t11846 = t511 * t3718;
    let t11849 = t2860 * t1544;
    let t11854 = t1148 * t3697;
    (t11827, t11834, t11837, t11840, t11843, t11846, t11849, t11854)
}
