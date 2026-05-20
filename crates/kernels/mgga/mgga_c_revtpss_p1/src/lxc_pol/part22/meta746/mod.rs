//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta746 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2815;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2816;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2817;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta746<F: Float>(t2439: F, t2912: F, t2915: F, t2909: F, t11821: F, t240: F, t2851: F, t25273: F, t268: F, t271: F, t2435: F, t2863: F, t2854: F, t11852: F, t159: F, t907: F, t9292: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41281, t41285, t41287, t41294, t41296, t41306) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2815::<F>(t2439, t2912, t2915, t2909, t11821, t240, t2851, t25273, t268, t271);
        let (t41307, t41329, t41330) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2816::<F>(t41306, t2435, t2863);
        let t41332 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2817::<F>(t2435, t2854);
        let (t41339, t41361) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2818::<F>(t11852, t159, t907, t9292);
    (t41281, t41285, t41287, t41294, t41296, t41306, t41307, t41329, t41330, t41332, t41339, t41361)
}
