//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1707;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1708;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta475<F: Float>(t2061: F, t25402: F, t7056: F, t10073: F, t26544: F, t7064: F, t7384: F, t887: F, t689: F, t7399: F, t786: F, t789: F, t2062: F, t2453: F, t2458: F, t2411: F, t7427: F, t11064: F, t2070: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26554, t26555, t26557, t26558, t26560, t26561, t26563, t26564) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1707::<F>(t2061, t25402, t7056, t10073, t26544, t7064, t7384, t887, t689, t7399, t786, t789);
        let (t26576, t26578, t26585) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1708::<F>(t2062, t2453, t2458, t2411, t7427);
        let t26590 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1709::<F>(t11064, t2070);
    (t26554, t26555, t26557, t26558, t26560, t26561, t26563, t26564, t26576, t26578, t26585, t26590)
}
