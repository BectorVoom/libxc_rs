//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta796 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2878;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2879;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta796(t41880: f64, t4595: f64, t15513: f64, t914: f64, t936: f64, t15416: f64, t2919: f64, t2923: f64, t4587: f64, t2927: f64, t11380: f64, t4590: f64, t11384: f64, t1596: f64, t11388: f64, t52201: f64, t52204: f64, t52207: f64, t52209: f64, t52211: f64, t1610: f64, t41571: f64, t11289: f64, t4632: f64, t11510: f64, t1633: f64, t41224: f64, t981: f64, t15573: f64, t3022: f64, t11466: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52213, t52216, t52218, t52221, t52223) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2878(t41880, t4595, t15513, t914, t936, t15416, t2919, t2923, t4587, t2927, t11380, t4590);
        let (t52226, t52227) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2879(t11384, t1596, t11388, t52201, t52204, t52207, t52209, t52211, t52213, t52216, t52218, t52221, t52223);
        let (t52229, t52231, t52235, t52237, t52238) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2880(t1610, t41571, t11289, t4632, t11510, t1633, t41224, t981, t15573, t3022, t11466, t300);
    (t52213, t52216, t52218, t52221, t52223, t52226, t52227, t52229, t52231, t52235, t52237, t52238)
}
