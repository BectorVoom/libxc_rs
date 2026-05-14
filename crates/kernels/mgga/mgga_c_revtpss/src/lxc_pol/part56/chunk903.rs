//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 903/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk903<F: Float>(t44125: F, t482: F, t675: F, t828: F, t12625: F, t458: F, t13180: F, t493: F, t10308: F, t599: F, t90: F, t29: F, t560: F, t9655: F, t4146: F, t550: F, t9794: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44126 = 1.0 / t44125;
    let t44545 = t675 * t482;
    let t44546 = t828 * t44545;
    let t44841 = 1.0 / t12625 / t458;
    let t45551 = 1.0 / t13180 / t493;
    let t45963 = t599 * t10308;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = 1.0 / t9655 / t560;
    let t47671 = t4146 * t4146;
    let t47672 = 1.0 / t47671;
    let t49068 = t9794 * t550;
    (t44126, t44545, t44546, t44841, t45551, t45963, t45972, t46361, t47672, t49068)
}
