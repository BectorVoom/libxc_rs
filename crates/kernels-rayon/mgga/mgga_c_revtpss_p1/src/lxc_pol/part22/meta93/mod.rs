//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta93 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk662;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk663;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk664;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk665;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk666;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk667;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta93(t2289: f64, t2270: f64, t2276: f64, t2279: f64, t2283: f64, t2286: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64, t38: f64, t45: f64, t631: f64, t78: f64, t57: f64, t635: f64, t81: f64, t2251: f64, t2258: f64, t633: f64, t637: f64, t77: f64, t2252: f64, t2260: f64, t2263: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t5: f64, t2240: f64, t2242: f64, t2247: f64, t2248: f64, t603: f64, t644: f64, t91: f64, t117: f64, t116: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2290, t2291, t2292) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk662(t2289, t2270, t2276, t2279, t2283, t2286, t44, t49, t56, t614, t617, t38);
        let (t2297, t2299) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk663(t45, t631, t78);
        let (t2304, t2306) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk664(t57, t635, t81);
        let t2312 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk665(t2251, t2258, t2299, t2306, t633, t637, t77);
        let t2315 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk666(t2252, t2260, t2263, t2292, t2312, t608, t628, t641, t71, t85);
        let (t2319, t2320, t2322) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk667(t5, t2240, t2242, t2247, t2248, t2315, t603, t644, t91, t117, t116, t648);
    (t2290, t2291, t2292, t2297, t2299, t2304, t2306, t2312, t2315, t2319, t2320, t2322)
}
