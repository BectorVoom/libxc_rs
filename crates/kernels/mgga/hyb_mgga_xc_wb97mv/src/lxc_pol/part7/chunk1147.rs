//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1147/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1147<F: Float>(t2688: F, t2692: F, t1099: F, t23583: F, t1085: F, t23588: F, t2702: F, t222: F, t464: F, t7615: F, t7684: F, t2709: F, t2776: F, t2697: F, t7706: F, t2701: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23638 = t2688 * t2688;
    let t23639 = 1.0 / t23638;
    let t23641 = t2692 * t2692;
    let t23642 = 1.0 / t23641;
    let t23645 = 0.91082604192152556044e5 * t1099 * t23639 * t23583 * t23642;
    let t23652 = 0.35089341735807877242e1 * t1099 * t2702 * t23588 * t1085;
    let t23653 = t464 * t222;
    let t23656 = 0.1301229756036208781e0 * t23653 * t7684 * t7615;
    let t23660 = 120.0 * t2709 * t2776;
    let t23661 = t2697 * t7706;
    let t23664 = 1.0 / t2688 / t2701;
    (t23639, t23642, t23645, t23652, t23653, t23656, t23660, t23661, t23664)
}
