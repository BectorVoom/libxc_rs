//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1149/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1149<F: Float>(t2731: F, t1026: F, t1064: F, t1069: F, t1070: F, t1071: F, t1084: F, t23547: F, t23552: F, t23606: F, t23608: F, t23611: F, t23614: F, t23617: F, t23622: F, t23624: F, t23626: F, t23628: F, t23631: F, t23674: F, t2683: F, t2690: F, t2693: F, t2728: F, t2729: F, t2730: F, t2741: F, t2745: F, t2746: F, t2749: F, t2750: F, t2764: F, t2771: F, t2772: F, t466: F, t7523: F, t7535: F, t7546: F, t7562: F, t7565: F, t7592: F, t7598: F, t7601: F, t7620: F, t7633: F, t7765: F, t7770: F) -> (F, F) {
    let t23697 = t2731 * t2731;
    let t23749 = -0.19263893255070628431e1 * t1026 * t7770 + 0.1301229756036208781e0 * t1026 * t7765 - 0.6609050294782684211e1 * t1026 * t2746 * t1069 * t7546 - t23547 + t23552 + 0.11579025239058625248e4 * t7598 * t23697 * t2749 + 0.41096e0 * t1026 * t2729 * t2741 * t1071 + 1.0 * t1064 * (-0.39219166666666666667e1 * t23606 + 0.376504e2 * t23608 - 0.13944592592592592593e2 * t23611 + 0.12201518518518518519e2 * t23614 + 0.5356037037037037037e1 * t23617 + 0.14025833333333333333e0 * t23622 - 0.22441333333333333332e1 * t23624 + 0.24934814814814814815e1 * t23626 + 0.21817962962962962963e1 * t23628 + 0.16979925925925925926e1 * t23631) * t1070 - 0.24828486201251232145e5 * t466 / t2745 / t2728 * t23697 * t7601 + t23674 - 0.62337092780453269531e3 * t7592 * t2772 * t2683 + 0.69263436422725855036e2 * t2771 * t7523 * t2693 * t1084 + 0.61524113149298439947e4 * t7565 * t2690 * t7535 * t2683 - 0.46785788981077169656e1 * t2764 * t7562 * t1084 - 0.11579025239058625248e4 * t7620 * t2750 * t2741 - 8.0 * t2730 * t1071 * t7633 + 0.12414243100625616072e5 * t7598 * t2741 * t7601 * t2731;
    (t23697, t23749)
}
