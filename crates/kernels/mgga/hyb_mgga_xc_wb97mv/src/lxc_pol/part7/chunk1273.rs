//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1273/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1273<F: Float>(t4188: F, t6862: F, t2200: F, t6859: F, t10940: F, t6914: F, t2245: F, t3373: F, t9111: F, t10944: F, t22649: F, t10943: F, t2239: F, t22654: F, t22656: F, t4162: F) -> (F, F, F, F, F, F) {
    let t31207 = t4188 * t6862;
    let t31210 = 0.51726012919273400301e3 * t6859 * t31207 * t2200;
    let t31212 = 0.64327917994770140268e2 * t6914 * t10940;
    let t31215 = 0.32163958997385070134e2 * t2245 * t3373 * t9111;
    let t31217 = 0.1034520258385468006e4 * t22649 * t10944;
    let t31220 = 0.51726012919273400301e3 * t6859 * t10943 * t2239;
    let t31224 = 0.24955700379505800916e5 * t22654 * t4162 * t22656 * t2200;
    (t31210, t31212, t31215, t31217, t31220, t31224)
}
