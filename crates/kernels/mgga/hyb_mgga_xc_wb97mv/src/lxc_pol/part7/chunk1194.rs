//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1194/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1194<F: Float>(t3638: F, t7536: F, t2694: F, t9696: F, t7746: F, t9629: F, t2685: F, t1089: F, t458: F, t9685: F, t1508: F, t6175: F, t1045: F, t3644: F, t3652: F, t1046: F, t9694: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27685 = t3638 * t7536;
    let t27687 = t9696 * t2694;
    let t27690 = t9629 * t7746;
    let t27692 = t9696 * t2685;
    let t27695 = t458 * t9685 * t1089;
    let t27697 = t6175 * t1508;
    let t27719 = 32.0 * t3644 * t1045;
    let t27742 = 32.0 * t3652 * t1045;
    let t27749 = t1046 * t9694;
    (t27685, t27687, t27690, t27692, t27695, t27697, t27719, t27742, t27749)
}
