//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 871/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk871<F: Float>(t1036: F, t7577: F, t1020: F, t1018: F, t2663: F, t437: F, t1035: F, t2633: F, t2666: F, t449: F, t2688: F, t484: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7578 = t7577 * t1036;
    let t7580 = 1.0 * t1020 * t7578;
    let t7582 = 1.0 / t2663 / t1018;
    let t7583 = t437 * t7582;
    let t7584 = t2633 * t1035;
    let t7586 = 1.0 / t2666 / t449;
    let t7587 = t7584 * t7586;
    let t7589 = 0.51726012919273400301e3 * t7583 * t7587;
    let t7591 = 1.0 / t2688 / t484;
    (t7578, t7580, t7582, t7583, t7584, t7586, t7587, t7589, t7591)
}
