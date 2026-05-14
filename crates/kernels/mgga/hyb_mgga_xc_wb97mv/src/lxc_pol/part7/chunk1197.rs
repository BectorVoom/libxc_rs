//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1197/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1197<F: Float>(t2715: F, t3668: F, t2709: F, t3666: F, t1041: F, t9694: F, t1046: F, t9653: F, t9686: F, t10: F, t1096: F, t9685: F, t3638: F, t7765: F, t7770: F, t2782: F, t9616: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27798 = t2715 * t3668;
    let t27805 = t2709 * t3668;
    let t27807 = t2709 * t3666;
    let t27818 = t1041 * t9694;
    let t27820 = t1046 * t9653;
    let t27823 = t1046 * t9686;
    let t27828 = t9685 * t10 * t1096;
    let t27830 = t3638 * t7765;
    let t27832 = t3638 * t7770;
    let t27834 = t9616 * t2782;
    (t27798, t27805, t27807, t27818, t27820, t27823, t27828, t27830, t27832, t27834)
}
