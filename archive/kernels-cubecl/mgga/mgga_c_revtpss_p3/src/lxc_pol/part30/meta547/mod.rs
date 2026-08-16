//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta547<F: Float>(t13180: F, t493: F, t2240: F, t2246: F, t10308: F, t599: F, t90: F, t29: F, t560: F, t9655: F, t1389: F, t268: F) -> (F, F, F, F, F, F) {
        let (t45551, t45958, t45963, t45972, t46361, t46808) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1986::<F>(t13180, t493, t2240, t2246, t10308, t599, t90, t29, t560, t9655, t1389, t268);
    (t45551, t45958, t45963, t45972, t46361, t46808)
}
