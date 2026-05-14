//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 707/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk707<F: Float>(t3668: F, t458: F, t2696: F, t2698: F, t2706: F, t2711: F, t2714: F, t2720: F, t2756: F, t2777: F, t2779: F, t2784: F, t2786: F, t2788: F, t3666: F, t3664: F) -> (F, F) {
    let t3669 = t458 * t3668;
    let t3672 = -t2696 - 0.5848223622634646207e0 * t2698 + t2706 + t2711 - t2714 - t2720 + t2777 + t458 * t3666 + t3669 - 0.18311447306006545054e-3 * t2779 - t2756 + t2784 + t2786 - 4.0 * t2788;
    let t3673 = t3664 + t3672;
    (t3669, t3673)
}
