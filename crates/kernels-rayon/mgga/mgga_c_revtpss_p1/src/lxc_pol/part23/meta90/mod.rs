//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk618;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk619;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk620;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk621;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk622;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk623;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk624;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta90(t2236: f64, t25: f64, t599: f64, t602: f64, t89: f64, t90: f64, t29: f64, t2: f64, t580: f64, t47: f64, t59: f64, t239: f64, t64: f64, t45: f64, t631: f64, t78: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2237 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk618(t2236);
        let (t2239, t2242, t2246) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk619(t2237, t25, t599, t602, t89, t90);
        let t2247 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk620(t2246, t29);
        let t2255 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk621(t2, t580);
        let t2275 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk622(t47);
        let t2282 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk623(t59);
        let t2289 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk624(t239, t64);
        let (t2290, t2297, t2299) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk625(t2289, t45, t631, t78);
    (t2237, t2239, t2242, t2246, t2247, t2255, t2275, t2282, t2289, t2290, t2297, t2299)
}
