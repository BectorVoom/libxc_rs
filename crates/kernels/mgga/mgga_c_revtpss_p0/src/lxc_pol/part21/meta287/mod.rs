//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1524;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta287<F: Float>(t2240: F, t602: F, t2246: F, t599: F, t88: F, t89: F, t90: F, t29: F, t2248: F, t644: F, t2315: F, t606: F, t70: F, t72: F, t30: F, t33: F, t1927: F, t2258: F, t2251: F, t627: F, t9344: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10298, t10301, t10308, t10309, t10310, t10313, t10317) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1524::<F>(t2240, t602, t2246, t599, t88, t89, t90, t29, t2248, t644, t2315, t606, t70, t72);
        let (t10318, t10321, t10326) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1525::<F>(t30, t33, t1927, t2258, t2251, t627, t9344, zeta_threshold);
    (t10298, t10301, t10308, t10309, t10310, t10313, t10317, t10318, t10321, t10326)
}
