//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1195/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1195<F: Float>(t1041: F, t9686: F, t1508: F, t7482: F, t7487: F, t3638: F, t7525: F, t7755: F, t1101: F, t479: F, t9685: F, t2704: F, t9696: F, t7743: F, t9629: F, t7749: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27751 = t1041 * t9686;
    let t27753 = t7482 * t1508;
    let t27755 = t7487 * t1508;
    let t27761 = t3638 * t7525;
    let t27763 = t3638 * t7755;
    let t27766 = t9685 * t479 * t1101;
    let t27768 = t9696 * t2704;
    let t27772 = t9629 * t7743;
    let t27774 = t9629 * t7749;
    (t27751, t27753, t27755, t27761, t27763, t27766, t27768, t27772, t27774)
}
