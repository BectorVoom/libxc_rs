//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 772/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk772<F: Float>(t10191: F, t3240: F, t2452: F, t3234: F, t3259: F, t3239: F, t6773: F, t3258: F, t2437: F, t329: F, t6210: F, t2440: F, t3238: F, t7029: F, t2674: F, t282: F) -> (F, F, F, F, F, F, F, F) {
    let t10192 = t10191 * t3240;
    let t10194 = t3234 * t2452;
    let t10195 = t10194 * t3259;
    let t10197 = t3239 * t6773;
    let t10198 = t3258 * t10197;
    let t10201 = t2437 * t3240;
    let t10203 = t6210 * t329;
    let t10204 = t10203 * t2440;
    let t10205 = t3238 * t10204;
    let t10207 = t3239 * t7029;
    let t10208 = t3238 * t10207;
    let t10210 = t2674 * t282;
    (t10192, t10195, t10198, t10201, t10203, t10205, t10208, t10210)
}
