//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 619/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk619<F: Float>(t1127: F, t1128: F, t1132: F, t1136: F, t1158: F, t1161: F, t1164: F, t2839: F, t2896: F, t2900: F, t2902: F, t2906: F, t2910: F, t2915: F, t2923: F, t2928: F, t2931: F, t2934: F, t2943: F, t2946: F, t2947: F, t2953: F, t2957: F) -> (F,) {
    let t2960 = -0.24e-1 * t2900 * t2902 - 0.32e-1 * t1127 * t2906 + 0.32e-1 * t1132 * t2910 + 0.384e-6 * t1136 * t2896 - 0.216e0 * t2915 * t2902 - 0.96e-1 * t1158 * t2906 + 0.96e-1 * t1161 * t2910 - 0.88e-4 * t1136 * t2923 - 0.88e-4 * t1164 * t2923 + 0.6e-2 * t1127 * t2928 - 0.6e-2 * t1132 * t2931 - 0.128e-3 * t1136 * t2934 + 0.18e-1 * t1158 * t2928 - 0.18e-1 * t1161 * t2931 - 0.128e-3 * t1164 * t2934 + 0.29333333333333333333e-1 * t1164 * t2943 + 0.18e-1 * t2946 * t2947 + 0.29333333333333333333e-1 * t1136 * t2943 + 0.9e-1 * t2953 * t1128 * t2839 + 0.126e0 * t2957 * t2947;
    (t2960,)
}
