//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1012;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta275<F: Float>(t2453: F, t9792: F, t240: F, t2712: F, t3994: F, t2713: F, t3951: F, t3964: F, t785: F, t9731: F, t225: F, t4062: F, t3889: F, t543: F, t1386: F, t2482: F, t814: F, t136: F, t1412: F, t220: F, t124: F, t1398: F, t3938: F, t4003: F, t4056: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9793, t9794, t9796, t9799, t9802, t9804) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1012::<F>(t2453, t9792, t240, t2712, t3994, t2713, t3951, t3964, t785, t9731, t225, t4062);
        let (t9810, t9816, t9818, t9822, t9840) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1013::<F>(t3889, t543, t1386, t2482, t814, t136, t1412, t220, t124, t1398, t3938, t4003, t4056);
    (t9793, t9794, t9796, t9799, t9802, t9804, t9810, t9816, t9818, t9822, t9840)
}
