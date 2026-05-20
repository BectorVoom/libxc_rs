//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk814;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk815;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk816;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta162<F: Float>(t4343: F, t828: F, t855: F, t1544: F, t221: F, t2675: F, t2674: F, t1558: F, t243: F, t231: F, t2662: F, t2661: F, t1565: F, t2652: F, t1561: F, t2741: F, t241: F, t2719: F, t820: F, t72: F, t245: F, t125: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4345, t4349, t4350, t4352, t4353) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk814::<F>(t4343, t828, t855, t1544, t221, t2675, t2674, t1558, t243, t231);
        let (t4354, t4355, t4357, t4359, t4362) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk815::<F>(t2662, t4353, t2661, t1565, t2652, t1561, t2741, t241, t2719, t820);
        let t4364 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk816::<F>(t243, t72, t245);
        let t4365 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk817::<F>(t125, t1558);
    (t4345, t4349, t4350, t4352, t4353, t4354, t4355, t4357, t4359, t4362, t4364, t4365)
}
