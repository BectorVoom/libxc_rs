//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 811/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk811<F: Float>(t2437: F, t3240: F, t329: F, t6210: F, t2440: F, t3238: F, t3239: F, t7029: F, t2674: F, t282: F, t61: F, t2255: F, t3188: F, t770: F, t791: F, t2431: F, t3197: F) -> (F, F, F, F, F, F, F) {
    let t10201 = t2437 * t3240;
    let t10203 = t6210 * t329;
    let t10204 = t10203 * t2440;
    let t10205 = t3238 * t10204;
    let t10207 = t3239 * t7029;
    let t10208 = t3238 * t10207;
    let t10210 = t2674 * t282;
    let t10211 = t61 * t10210;
    let t10212 = t3188 * t2255;
    let t10213 = t10211 * t10212;
    let t10215 = t791 * t770;
    let t10216 = t3197 * t2431;
    (t10201, t10203, t10205, t10208, t10213, t10215, t10216)
}
