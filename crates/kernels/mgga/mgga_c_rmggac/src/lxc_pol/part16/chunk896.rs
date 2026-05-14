//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 896/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk896<F: Float>(t1652: F, t1971: F, t515: F, t615: F, t7230: F, t2144: F, t495: F, t6557: F, t1864: F, t209: F, t236: F, t36336: F, t476: F, t40231: F, t9222: F, t46454: F, t7192: F) -> (F, F, F, F, F) {
    let t47460 = t7230 * t1971 * t515 * t1652 * t615;
    let t47465 = t7230 * t1971 * t2144 * t6557 * t495;
    let t47471 = t36336 * t1971 * t236 * t1864 * t476 * t209;
    let t47473 = t9222 * t40231;
    let t47478 = t7192 * t46454;
    (t47460, t47465, t47471, t47473, t47478)
}
