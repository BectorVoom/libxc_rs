//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta542<F: Float>(t11119: F, t384: F, t11238: F, t196: F, t2240: F, t2246: F, t10308: F, t599: F, t90: F, t29: F, t560: F, t9655: F) -> (F, F, F, F, F, F) {
        let (t42066, t42859, t45958, t45963, t45972, t46361) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1991::<F>(t11119, t384, t11238, t196, t2240, t2246, t10308, t599, t90, t29, t560, t9655);
    (t42066, t42859, t45958, t45963, t45972, t46361)
}
