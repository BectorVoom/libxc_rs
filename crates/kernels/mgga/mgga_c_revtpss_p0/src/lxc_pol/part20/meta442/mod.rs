//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1687;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1688;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1689;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1690;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1691;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1692;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1693;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta442<F: Float>(t12: F, t14: F, t27: F, t10285: F, t596: F, t2231: F, t2237: F, t10293: F, t592: F, t25: F, t40649: F, t45927: F, t45929: F, t45931: F, t45933: F, t45935: F, t45937: F, t45939: F, t45941: F, t10296: F, t602: F, t2240: F, t2246: F, t10308: F, t599: F, t90: F, t29: F, t2248: F, t2315: F, t11149: F, t78: F, t12267: F, t81: F, t10321: F, t10326: F, t10336: F, t10380: F, t10381: F, t10389: F, t10392: F, t10398: F, t10401: F, t10407: F, t2251: F, t2258: F, t2260: F, t2263: F, t2291: F, t2292: F, t2299: F, t2306: F, t2312: F, t39443: F, t39449: F, t39457: F, t607: F, t608: F, t628: F, t633: F, t637: F, t641: F, t71: F, t77: F, t85: F, t46: F, t47: F, t58: F, t59: F, t2681: F, t64: F, t10345: F, t10355: F, t10357: F, t10360: F, t10361: F, t10364: F, t10368: F, t10372: F, t2270: F, t2275: F, t2276: F, t2279: F, t2282: F, t42748: F, t44: F, t48: F, t49: F, t56: F, t60: F, t614: F, t617: F, t10317: F, t10318: F, t10327: F, t10328: F, t10331: F, t1927: F, t2252: F, t2259: F, t36: F, t38: F, t606: F, t627: F, t6977: F, t70: F, t72: F, t10298: F, t10301: F, t10309: F, t10310: F, t10313: F, t10410: F, t2242: F, t2247: F, t603: F, t644: F, t91: F, t5: F, t117: F, t10414: F, t116: F, t2319: F, t2327: F, t2371: F, t112: F, t10199: F, t666: F, t2289: F, t2341: F) -> (F, F, F, F, F, F, F) {
        let t45953 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1687::<F>(t12, t14, t27, t10285, t596, t2231, t2237, t10293, t592, t25, t40649, t45927, t45929, t45931, t45933, t45935, t45937, t45939, t45941);
        let (t45955, t45958, t45963, t45972, t45973, t45979, t46001) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1688::<F>(t10296, t602, t2240, t2246, t10308, t599, t90, t29, t2248, t2315, t11149, t78);
        let t46034 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1689::<F>(t12267, t81, t10321, t10326, t10336, t10380, t10381, t10389, t10392, t10398, t10401, t10407, t2251, t2258, t2260, t2263, t2291, t2292, t2299, t2306, t2312, t39443, t39449, t39457, t46001, t607, t608, t628, t633, t637, t641, t71, t77, t85);
        let (t46089, t46091) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1690::<F>(t46, t47, t58, t59, t2681, t64, t10326, t10345, t10355, t10357, t10360, t10361, t10364, t10368, t10372, t2251, t2258, t2270, t2275, t2276, t2279, t2282, t39443, t39449, t39457, t42748, t44, t48, t49, t56, t60, t614, t617);
        let t46119 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1691::<F>(t10317, t10318, t10326, t10327, t10328, t10331, t1927, t2252, t2258, t2259, t2291, t2312, t36, t38, t39449, t39457, t46091, t606, t627, t641, t6977, t70, t72, t85);
        let t46123 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1692::<F>(t10298, t10301, t10309, t10310, t10313, t10410, t2242, t2247, t2248, t2315, t45953, t45955, t45958, t45963, t45972, t45973, t45979, t46034, t46119, t603, t644, t91);
        let (t46125, t46126, t46129, t46137, t46143, t46144, t46146) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1693::<F>(t5, t46123, t117, t10414, t116, t2319, t2327, t2371, t112, t46089, t10199, t666, t2289, t2341);
    (t46125, t46126, t46129, t46137, t46143, t46144, t46146)
}
