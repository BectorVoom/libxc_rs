//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 583/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk583<F: Float>(t2731: F, t2749: F, t1019: F, t566: F, t1037: F, t222: F, t1078: F, t2702: F, t479: F, t1085: F, t2690: F, t2683: F, t2689: F, t2693: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2750 = t2731 * t2749;
    let t2753 = t566 * t1019;
    let t2756 = 0.35616666666666666666e-1 * t222 * t2753 * t1037;
    let t2760 = t566 * t1078;
    let t2764 = t479 * t2702;
    let t2765 = t2690 * t1085;
    let t2768 = t2683 * t1085;
    let t2771 = t479 * t2689;
    let t2772 = t2690 * t2693;
    (t2750, t2753, t2756, t2760, t2764, t2765, t2768, t2771, t2772)
}
