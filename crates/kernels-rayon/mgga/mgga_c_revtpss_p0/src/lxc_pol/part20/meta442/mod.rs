//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1687;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1688;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1689;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1690;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1691;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1692;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1693;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta442(t12: f64, t14: f64, t27: f64, t10285: f64, t596: f64, t2231: f64, t2237: f64, t10293: f64, t592: f64, t25: f64, t40649: f64, t45927: f64, t45929: f64, t45931: f64, t45933: f64, t45935: f64, t45937: f64, t45939: f64, t45941: f64, t10296: f64, t602: f64, t2240: f64, t2246: f64, t10308: f64, t599: f64, t90: f64, t29: f64, t2248: f64, t2315: f64, t11149: f64, t78: f64, t12267: f64, t81: f64, t10321: f64, t10326: f64, t10336: f64, t10380: f64, t10381: f64, t10389: f64, t10392: f64, t10398: f64, t10401: f64, t10407: f64, t2251: f64, t2258: f64, t2260: f64, t2263: f64, t2291: f64, t2292: f64, t2299: f64, t2306: f64, t2312: f64, t39443: f64, t39449: f64, t39457: f64, t607: f64, t608: f64, t628: f64, t633: f64, t637: f64, t641: f64, t71: f64, t77: f64, t85: f64, t46: f64, t47: f64, t58: f64, t59: f64, t2681: f64, t64: f64, t10345: f64, t10355: f64, t10357: f64, t10360: f64, t10361: f64, t10364: f64, t10368: f64, t10372: f64, t2270: f64, t2275: f64, t2276: f64, t2279: f64, t2282: f64, t42748: f64, t44: f64, t48: f64, t49: f64, t56: f64, t60: f64, t614: f64, t617: f64, t10317: f64, t10318: f64, t10327: f64, t10328: f64, t10331: f64, t1927: f64, t2252: f64, t2259: f64, t36: f64, t38: f64, t606: f64, t627: f64, t6977: f64, t70: f64, t72: f64, t10298: f64, t10301: f64, t10309: f64, t10310: f64, t10313: f64, t10410: f64, t2242: f64, t2247: f64, t603: f64, t644: f64, t91: f64, t5: f64, t117: f64, t10414: f64, t116: f64, t2319: f64, t2327: f64, t2371: f64, t112: f64, t10199: f64, t666: f64, t2289: f64, t2341: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t45953 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1687(t12, t14, t27, t10285, t596, t2231, t2237, t10293, t592, t25, t40649, t45927, t45929, t45931, t45933, t45935, t45937, t45939, t45941);
        let (t45955, t45958, t45963, t45972, t45973, t45979, t46001) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1688(t10296, t602, t2240, t2246, t10308, t599, t90, t29, t2248, t2315, t11149, t78);
        let t46034 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1689(t12267, t81, t10321, t10326, t10336, t10380, t10381, t10389, t10392, t10398, t10401, t10407, t2251, t2258, t2260, t2263, t2291, t2292, t2299, t2306, t2312, t39443, t39449, t39457, t46001, t607, t608, t628, t633, t637, t641, t71, t77, t85);
        let (t46089, t46091) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1690(t46, t47, t58, t59, t2681, t64, t10326, t10345, t10355, t10357, t10360, t10361, t10364, t10368, t10372, t2251, t2258, t2270, t2275, t2276, t2279, t2282, t39443, t39449, t39457, t42748, t44, t48, t49, t56, t60, t614, t617);
        let t46119 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1691(t10317, t10318, t10326, t10327, t10328, t10331, t1927, t2252, t2258, t2259, t2291, t2312, t36, t38, t39449, t39457, t46091, t606, t627, t641, t6977, t70, t72, t85);
        let t46123 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1692(t10298, t10301, t10309, t10310, t10313, t10410, t2242, t2247, t2248, t2315, t45953, t45955, t45958, t45963, t45972, t45973, t45979, t46034, t46119, t603, t644, t91);
        let (t46125, t46126, t46129, t46137, t46143, t46144, t46146) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1693(t5, t46123, t117, t10414, t116, t2319, t2327, t2371, t112, t46089, t10199, t666, t2289, t2341);
    (t46125, t46126, t46129, t46137, t46143, t46144, t46146)
}
