//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 343/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk343<F: Float>(t1111: F, t1112: F, t1114: F, t1115: F, t1117: F, t1123: F, t1127: F, t1129: F, t1132: F, t1133: F, t1136: F, t1138: F, t1144: F, t1148: F, t1153: F, t1158: F, t1161: F, t1164: F, t505: F, t507: F, t511: F, t513: F, t529: F, t532: F) -> (F,) {
    let t1169 = t1112 * t507 - t505 * t1115 + 2.0 * t1117 * t513 * t1111 - 2.0 * t511 * t1123 + 0.6e-2 * t1127 * t1129 - 0.6e-2 * t1132 * t1133 - 0.8e-2 * t1136 * t1138 + 0.24e-4 * t1136 * t1144 + 6.0 * t1148 * t532 * t1111 - 6.0 * t529 * t1153 * t1114 + 0.18e-1 * t1158 * t1129 - 0.18e-1 * t1161 * t1133 - 0.8e-2 * t1164 * t1138 + 0.24e-4 * t1164 * t1144;
    (t1169,)
}
