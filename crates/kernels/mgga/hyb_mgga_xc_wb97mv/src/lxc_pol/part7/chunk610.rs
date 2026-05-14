//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 610/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk610<F: Float>(t2894: F, t2895: F, t1111: F, t1112: F, t1114: F, t1115: F, t1117: F, t1122: F, t1148: F, t1153: F, t1164: F, t2817: F, t2819: F, t2823: F, t2825: F, t2828: F, t2832: F, t2839: F, t2840: F, t2848: F, t2849: F, t2853: F, t2857: F, t2860: F, t2870: F, t2874: F, t2877: F, t2880: F, t2885: F, t2887: F, t2890: F, t505: F, t507: F, t511: F, t513: F, t523: F, t529: F, t532: F) -> (F, F) {
    let t2896 = t2894 * t2895;
    let t2899 = 0.96e-4 * t2817 * t2819 - 0.96e-4 * t2823 * t2825 + 0.288e-3 * t2828 * t2819 - 0.288e-3 * t2832 * t2825 - 8.0 * t1117 * t1122 * t1111 * t1114 + 0.6e-2 * t2840 * t523 - 72.0 * t1148 * t1153 * t1111 * t1114 + 42.0 * t529 * t2848 * t2849 + 2.0 * t505 * t2853 + 6.0 * t511 * t2857 + 30.0 * t2860 * t532 * t2839 + 6.0 * t1148 * t2870 - 6.0 * t529 * t2874 + 2.0 * t1117 * t2877 - 2.0 * t511 * t2880 - 2.0 * t1112 * t1115 - t505 * t2885 + 2.0 * t2887 * t513 + t2890 * t507 + 0.384e-6 * t1164 * t2896;
    (t2896, t2899)
}
