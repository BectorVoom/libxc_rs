//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 832/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk832<F: Float>(t2035: F, t2040: F, t6725: F, t10: F, t6536: F, t138: F, t2033: F, t2015: F, t679: F, t2022: F, t676: F, t215: F, t3003: F, t136: F, t222: F, t226: F, t6129: F) -> (F, F, F, F, F, F, F, F) {
    let t6727 = t2035 * t6725 * t2040;
    let t6733 = t6536 * t10;
    let t6736 = 1.0 / t138 / t2033;
    let t6741 = t2015 * t679;
    let t6743 = t676 * t2022;
    let t6745 = t3003 * t215;
    let t6747 = 5.0 / 288.0 * t136 * t6745;
    let t6759 = t222 * t6129 * t226;
    (t6727, t6733, t6736, t6741, t6743, t6745, t6747, t6759)
}
