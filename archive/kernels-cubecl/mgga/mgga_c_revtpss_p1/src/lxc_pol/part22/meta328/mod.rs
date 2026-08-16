//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1781;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta328<F: Float>(t2648: F, t2741: F, t2710: F, t826: F, t9732: F, t234: F, t2735: F, t10631: F, t808: F, t2699: F, t798: F, t802: F, t2703: F, t2707: F, t159: F, t853: F, t216: F, t2729: F, t794: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10881, t10885, t10886) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1780::<F>(t2648, t2741, t2710, t826, t9732, t234, t2735);
        let (t10887, t10888, t10890) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1781::<F>(t10631, t808, t10886, t2699, t798);
        let (t10891, t10893, t10899, t10900, t10905) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1782::<F>(t10890, t802, t2703, t2707, t159, t853, t216, t2729, t794);
    (t10881, t10885, t10886, t10887, t10888, t10890, t10891, t10893, t10899, t10900, t10905)
}
