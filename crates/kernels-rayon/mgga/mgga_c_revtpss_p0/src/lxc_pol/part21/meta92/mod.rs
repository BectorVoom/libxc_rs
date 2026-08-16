//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk640;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk641;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk642;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk643;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk644;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta92(t2251: f64, t2282: f64, t2258: f64, t60: f64, t239: f64, t64: f64, t2270: f64, t2276: f64, t2279: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64, t38: f64, t45: f64, t631: f64, t78: f64, t57: f64, t635: f64, t81: f64, t633: f64, t637: f64, t77: f64, t2252: f64, t2260: f64, t2263: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2283, t2286, t2289) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk640(t2251, t2282, t2258, t60, t239, t64);
        let (t2290, t2291) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk641(t2289, t2270, t2276, t2279, t2283, t2286, t44, t49, t56, t614, t617);
        let (t2292, t2297, t2299) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk642(t2291, t38, t45, t631, t78);
        let (t2304, t2306) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk643(t57, t635, t81);
        let t2312 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk644(t2251, t2258, t2299, t2306, t633, t637, t77);
        let t2315 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk645(t2252, t2260, t2263, t2292, t2312, t608, t628, t641, t71, t85);
    (t2283, t2286, t2289, t2290, t2291, t2292, t2297, t2299, t2304, t2306, t2312, t2315)
}
