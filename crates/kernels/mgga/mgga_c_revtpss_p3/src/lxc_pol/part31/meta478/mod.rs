//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1752;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta478<F: Float>(t159: F, t2698: F, t218: F, t816: F, t228: F, t7021: F, t802: F, t7043: F, t826: F, t2736: F, t7082: F, t72: F, t686: F, t7058: F, t2453: F, t7057: F, t136: F, t1958: F, t2457: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25273, t25276, t25277, t25278, t25282, t25284, t25295) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1752::<F>(t159, t2698, t218, t816, t228, t7021, t802, t7043, t826, t2736, t7082, t72);
        let (t25296, t25297, t25299, t25300, t25301) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1753::<F>(t25295, t686, t7058, t2453, t7057, t136, t1958, t2457);
    (t25273, t25276, t25277, t25278, t25282, t25284, t25295, t25296, t25297, t25299, t25300, t25301)
}
