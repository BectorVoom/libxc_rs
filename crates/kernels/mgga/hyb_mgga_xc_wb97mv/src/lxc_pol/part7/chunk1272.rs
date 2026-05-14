//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1272/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1272<F: Float>(t11158: F, t11162: F, t11174: F, t11175: F, t22440: F, t22556: F, t22558: F, t2263: F, t2278: F, t2284: F, t26834: F, t26846: F, t26850: F, t31156: F, t31159: F, t31162: F, t31165: F, t31167: F, t31168: F, t31175: F, t3406: F, t3421: F, t3440: F, t4202: F, t6871: F, t6903: F, t827: F, t9049: F, t9068: F, t9077: F, t9142: F, t9145: F, t9148: F, t9152: F) -> (F,) {
    let t31205 = -t31156 - t31159 - t31162 + t31165 - t31167 + 0.64327917994770140268e2 * t2284 * t31168 * t827 + 0.32163958997385070134e2 * t2284 * t11158 * t2278 + 0.2069040516770936012e4 * t6903 * t31175 * t2263 + 0.12865583598954028054e3 * t6871 * t11162 + 0.64327917994770140268e2 * t2284 * t3406 * t9049 + 0.4138081033541872024e4 * t22440 * t11175 + 0.2069040516770936012e4 * t6903 * t11174 * t2278 + 0.19964560303604640732e6 * t22556 * t4202 * t22558 * t2263 - 0.46785788981077169656e1 * t26846 * t3421 - 0.46785788981077169656e1 * t9068 * t9142 - 0.23392894490538584828e1 * t9068 * t9145 - 0.2077903092681775651e3 * t26850 * t9148 + 0.69263436422725855034e2 * t26834 * t3440 + 0.69263436422725855034e2 * t9077 * t9152;
    (t31205,)
}
