//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1991/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1991<F: Float>(t11119: F, t384: F, t11238: F, t196: F, t2240: F, t2246: F, t10308: F, t599: F, t90: F, t29: F, t560: F, t9655: F) -> (F, F, F, F, F, F) {
    let t42066 = F::cast_from(1.0_f64) / t11119 / t384;
    let t42859 = F::cast_from(1.0_f64) / t11238 / t196;
    let t45958 = t2240 * t2246;
    let t45963 = t599 * t10308;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = F::cast_from(1.0_f64) / t9655 / t560;
    (t42066, t42859, t45958, t45963, t45972, t46361)
}
