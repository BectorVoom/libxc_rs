//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2747;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2748;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta715<F: Float>(t215: F, t2722: F, t231: F, t268: F, t2798: F, t2645: F, t14545: F, t251: F, t4503: F, t860: F, t786: F, t10115: F, t883: F, t2710: F, t2793: F, t39494: F, t2804: F, t874: F, t9288: F, t10535: F, t281: F, t68: F, t211: F, t9644: F, t209: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39583, t39586, t39595, t39597, t39608, t39609, t39624) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2747::<F>(t215, t2722, t231, t268, t2798, t2645, t14545, t251, t4503, t860, t786, t10115, t883);
        let (t39633, t39635, t39640, t39643) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2748::<F>(t2710, t2793, t39494, t2804, t874, t9288, t10535, t231, t2645, t281, t68, t211, t9644);
        let t39644 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2749::<F>(t209, t39643);
    (t39583, t39586, t39595, t39597, t39608, t39609, t39624, t39633, t39635, t39640, t39644)
}
