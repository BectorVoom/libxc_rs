//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1269/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1269<F: Float>(t10930: F, t22407: F, t2239: F, t4192: F, t6919: F, t10943: F, t2200: F, t22399: F, t10933: F, t6937: F, t11035: F, t2199: F, t808: F, t4189: F, t10936: F, t10937: F, t6914: F) -> (F, F, F, F, F, F, F, F) {
    let t31132 = 0.1929837539843104208e3 * t22407 * t10930;
    let t31135 = 0.96491876992155210402e2 * t6919 * t4192 * t2239;
    let t31138 = 0.62071215503128080361e4 * t22399 * t10943 * t2200;
    let t31140 = 4.0 * t6937 * t10933;
    let t31143 = 4.0 * t2199 * t11035 * t808;
    let t31146 = 2.0 * t2199 * t4189 * t2239;
    let t31149 = 0.96491876992155210402e2 * t6919 * t10936 * t2200;
    let t31151 = 0.32163958997385070134e2 * t6914 * t10937;
    (t31132, t31135, t31138, t31140, t31143, t31146, t31149, t31151)
}
