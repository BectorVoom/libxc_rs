//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta796 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2878;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2879;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta796<F: Float>(t41880: F, t4595: F, t15513: F, t914: F, t936: F, t15416: F, t2919: F, t2923: F, t4587: F, t2927: F, t11380: F, t4590: F, t11384: F, t1596: F, t11388: F, t52201: F, t52204: F, t52207: F, t52209: F, t52211: F, t1610: F, t41571: F, t11289: F, t4632: F, t11510: F, t1633: F, t41224: F, t981: F, t15573: F, t3022: F, t11466: F, t300: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52213, t52216, t52218, t52221, t52223) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2878::<F>(t41880, t4595, t15513, t914, t936, t15416, t2919, t2923, t4587, t2927, t11380, t4590);
        let (t52226, t52227) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2879::<F>(t11384, t1596, t11388, t52201, t52204, t52207, t52209, t52211, t52213, t52216, t52218, t52221, t52223);
        let (t52229, t52231, t52235, t52237, t52238) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2880::<F>(t1610, t41571, t11289, t4632, t11510, t1633, t41224, t981, t15573, t3022, t11466, t300);
    (t52213, t52216, t52218, t52221, t52223, t52226, t52227, t52229, t52231, t52235, t52237, t52238)
}
