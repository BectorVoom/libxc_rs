//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1980/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1980<F: Float>(t3800: F, t12625: F, t458: F, t13180: F, t493: F, t10308: F, t599: F, t90: F, t29: F, t560: F, t9655: F, t1389: F, t268: F) -> (F, F, F, F, F, F, F) {
    let t44125 = t3800 * t3800;
    let t44126 = F::new(1.0) / t44125;
    let t44841 = F::new(1.0) / t12625 / t458;
    let t45551 = F::new(1.0) / t13180 / t493;
    let t45963 = t599 * t10308;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = F::new(1.0) / t9655 / t560;
    let t46808 = t1389 * t268;
    (t44126, t44841, t45551, t45963, t45972, t46361, t46808)
}
