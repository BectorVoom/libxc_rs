//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta571<F: Float>(t101218: F, t2047: F, t28154: F, t95296: F, t28147: F, t95319: F, t28150: F, t7348: F, t25162: F, t116: F, t28651: F, t2106: F, t47672: F) -> (F, F, F, F, F, F, F) {
        let (t101938, t101955, t101969, t101970, t101972, t102019, t102070) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1895::<F>(t101218, t2047, t28154, t95296, t28147, t95319, t28150, t7348, t25162, t116, t28651, t2106, t47672);
    (t101938, t101955, t101969, t101970, t101972, t102019, t102070)
}
