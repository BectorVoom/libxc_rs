//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 817/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk817<F: Float>(t2268: F, t2343: F, t46945: F, t39656: F, t39657: F, t42111: F, t42113: F, t42114: F, t42117: F, t42118: F, t42119: F, t42120: F, t42121: F, t42122: F) -> (F, F) {
    let t46947 = t2268 * t2343 * t46945;
    let t46952 = t42111 - t42113 + t42114 / 2.0 + t39656 - t39657 + t42117 + t42118 - t42119 + t42120 - t42121 - t42122;
    (t46947, t46952)
}
