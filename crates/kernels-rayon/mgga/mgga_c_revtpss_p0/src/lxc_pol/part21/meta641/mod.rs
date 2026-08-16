//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2418;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2419;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2420;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2421;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2422;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2423;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2424;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2425;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2426;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta641(t41306: f64, t2435: f64, t2863: f64, t2854: f64, t11170: f64, t689: f64, t11146: f64, t11852: f64, t159: f64, t907: f64, t9292: f64, t2859: f64, t11166: f64, t11157: f64, t11152: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41329, t41330) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2418(t41306, t2435, t2863);
        let t41332 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2419(t2435, t2854);
        let t41334 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2420(t11170, t689);
        let t41336 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2421(t11146, t689);
        let (t41339, t41361) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2422(t11852, t159, t907, t9292);
        let t41363 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2423(t2435, t2859);
        let t41365 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2424(t11166, t689);
        let t41367 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2425(t11157, t689);
        let t41369 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2426(t11152, t689);
    (t41329, t41330, t41332, t41334, t41336, t41339, t41361, t41363, t41365, t41367, t41369)
}
