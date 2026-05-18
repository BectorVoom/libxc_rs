//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1062/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1062<F: Float>(t13363: F, t6416: F, t13242: F, t3116: F, t6331: F, t3786: F, t3912: F, t6158: F, t2118: F, t360: F, t13549: F, t21536: F) -> (F, F, F, F, F) {
    let t46280 = t6416 * t13363;
    let t46324 = t3116 * t6331 * t13242;
    let t46327 = t3912 * t6158 * t3786;
    let t46382 = t3912 * t2118 * t3786 * t360;
    let t46399 = t21536 * t13549;
    (t46280, t46324, t46327, t46382, t46399)
}
