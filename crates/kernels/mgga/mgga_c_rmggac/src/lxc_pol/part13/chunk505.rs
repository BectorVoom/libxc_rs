//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 505/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk505<F: Float>(t2069: F, t352: F, t262: F, t7204: F, t2160: F, t2165: F, t638: F, t2169: F, t1288: F, t71: F, t131: F, t639: F, t2164: F, t356: F, t1276: F, t640: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7205 = t2069 * t352;
    let t7206 = t262 * t7205;
    let t7207 = t7204 * t7206;
    let t7210 = t638 * t2160 * t2165;
    let t7213 = t638 * t2160 * t2169;
    let t7215 = t71 * t1288;
    let t7216 = t7215 * t131;
    let t7218 = t638 * t639 * t7216;
    let t7220 = t2164 * t356;
    let t7222 = t638 * t639 * t7220;
    let t7224 = t640 * t1276;
    (t7205, t7206, t7207, t7210, t7213, t7215, t7216, t7218, t7220, t7222, t7224)
}
