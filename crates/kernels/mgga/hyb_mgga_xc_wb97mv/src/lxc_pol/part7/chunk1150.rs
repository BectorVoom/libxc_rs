//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1150/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1150<F: Float>(t1069: F, t1085: F, t1846: F, t222: F, t23583: F, t23588: F, t2683: F, t2689: F, t2693: F, t2702: F, t2724: F, t2732: F, t2741: F, t2742: F, t2746: F, t2747: F, t2749: F, t2750: F, t2760: F, t2764: F, t2765: F, t2768: F, t2771: F, t2772: F, t566: F, t7559: F, t7562: F, t7591: F, t7592: F, t7593: F, t7597: F, t7602: F, t7633: F, t7634: F, t7640: F, t7658: F, t7662: F, t7669: F, t7673: F) -> (F,) {
    let t23807 = 0.12865583598954028054e3 * t2747 * t7633 * t2749 * t1069 + 36.0 * t2747 * t2732 * t2741 - 0.14035736694323150897e2 * t7592 * t23583 * t1085 - 0.35089341735807877242e1 * t2764 * t23588 * t1085 + 0.51947577317044391277e2 * t2771 * t23588 * t2693 + 0.21053605041484726346e2 * t2771 * t2765 * t2683 + 0.43374325201206959368e-1 * t222 * t7662 * t2768 - 0.21687162600603479684e-1 * t222 * t2760 * t7562 - 0.14171548179536397724e3 * t222 * t566 * t7597 * t7602 + 0.13698666666666666666e0 * t222 * t7673 * t2742 - 0.68493333333333333332e-1 * t222 * t2724 * t7634 - 0.41096e0 * t222 * t7669 * t7640 - 0.1301229756036208781e0 * t222 * t7658 * t7559 + 0.12842595503380418954e1 * t222 * t1846 * t2689 * t2772 + 0.38527786510141256862e1 * t222 * t566 * t7591 * t7593 - 0.86748650402413918736e-1 * t222 * t1846 * t2702 * t2765 + 0.44060335298551228073e1 * t222 * t1846 * t2746 * t2750;
    (t23807,)
}
