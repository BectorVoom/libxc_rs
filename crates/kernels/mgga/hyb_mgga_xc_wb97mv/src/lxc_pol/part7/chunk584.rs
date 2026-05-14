//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 584/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk584<F: Float>(t1064: F, t1071: F, t1079: F, t1086: F, t221: F, t222: F, t2636: F, t2647: F, t2662: F, t2670: F, t2720: F, t2724: F, t2730: F, t2732: F, t2742: F, t2747: F, t2750: F, t2756: F, t2760: F, t2764: F, t2765: F, t2768: F, t2771: F, t2772: F, t475: F, t488: F) -> (F,) {
    let t2775 = -0.70983522622222222221e-3 * t221 * t2647 * t475 - 0.34246666666666666666e-1 * t222 * t2724 * t1071 - 2.0 * t2730 * t2732 + 1.0 * t1064 * t2742 + 0.32163958997385070134e2 * t2747 * t2750 + t2720 + t2756 + t2636 - t2662 - t2670 - 0.24415263074675393405e-3 * t221 * t2647 * t488 - 0.10843581300301739842e-1 * t222 * t2760 * t1086 - 0.11696447245269292414e1 * t2764 * t2765 + 0.5848223622634646207e0 * t1079 * t2768 + 0.17315859105681463759e2 * t2771 * t2772;
    (t2775,)
}
