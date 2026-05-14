//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1271/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1271<F: Float>(t11034: F, t2247: F, t2245: F, t808: F, t10936: F, t2239: F, t2200: F, t4189: F, t4163: F, t6919: F, t9120: F, t9187: F, t11067: F, t2286: F, t4215: F, t6905: F) -> (F, F, F, F, F, F, F) {
    let t31153 = t11034 * t2247;
    let t31156 = 0.32163958997385070134e2 * t2245 * t31153 * t808;
    let t31159 = 0.16081979498692535067e2 * t2245 * t10936 * t2239;
    let t31162 = 6.0 * t2245 * t4189 * t2200;
    let t31165 = 24.0 * t6919 * t4163 * t2200;
    let t31167 = 12.0 * t9120 * t9187;
    let t31168 = t11067 * t2286;
    let t31175 = t4215 * t6905;
    (t31156, t31159, t31162, t31165, t31167, t31168, t31175)
}
