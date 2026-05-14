//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1111/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1111<F: Float>(t1882: F, t26838: F, t12001: F, t27273: F, t26947: F, t6701: F, t8232: F, t26957: F, t1384: F, t39652: F, t6645: F, t26965: F, t6641: F, t26932: F, t8392: F, t27316: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t107477 = 2.0 / 9.0 * t1882 * t26838;
    let t107478 = t12001 * t27273;
    let t107499 = 2.0 / 9.0 * t1882 * t26947;
    let t107519 = t8232 * t6701;
    let t107533 = 2.0 / 9.0 * t1882 * t26957;
    let t107542 = t39652 * t1384;
    let t107547 = t8232 * t6645;
    let t107552 = 2.0 / 9.0 * t1882 * t26965;
    let t107563 = t8232 * t6641;
    let t107566 = 2.0 / 27.0 * t8392 * t26932;
    let t107573 = 2.0 / 9.0 * t1882 * t27316;
    (t107477, t107478, t107499, t107519, t107533, t107542, t107547, t107552, t107563, t107566, t107573)
}
