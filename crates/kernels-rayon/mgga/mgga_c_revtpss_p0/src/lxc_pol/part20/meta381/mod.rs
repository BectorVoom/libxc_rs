//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1383;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1384;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1385;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1386;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta381(t240: f64, t9731: f64, t10760: f64, t2664: f64, t10293: f64, t124: f64, t212: f64, t800: f64, t810: f64, t820: f64, t849: f64, t9948: f64, t857: f64, t10722: f64, t2479: f64, t14832: f64, t2430: f64, t2475: f64, t2661: f64, t775: f64, t2699: f64, t2729: f64, t2732: f64, t235: f64, t4503: f64, t2453: f64, t10728: f64, t9794: f64, t10886: f64, t40236: f64, t808: f64, t123: f64, t125: f64, t2452: f64, t40633: f64, t10785: f64, t10943: f64, t14791: f64, t2730: f64, t40240: f64, t40655: f64, t40748: f64, t40750: f64, t40753: f64, t40759: f64, t40761: f64, t4362: f64, t4364: f64, t4366: f64, t10766: f64, t10811: f64, t10788: f64, t14923: f64, t10799: f64, t10759: f64, t2735: f64, t40628: f64, t854: f64, t10890: f64, t2707: f64, t10896: f64, t2703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40763, t40765, t40769, t40771, t40781) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1383(t240, t9731, t10760, t2664, t10293, t124, t212, t800, t810, t820, t849, t9948);
        let (t40782, t40784, t40789, t40792) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1384(t40781, t857, t10722, t2479, t14832, t2430, t2475, t2661, t775, t2699, t2729, t2732);
        let (t40801, t40804, t40810) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1385(t235, t4503, t2453, t10728, t9794, t10886, t40236, t808, t123, t125, t2452, t40633, t810);
        let t40811 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1386(t10785, t10943, t124, t14791, t2730, t40240, t40655, t40748, t40750, t40753, t40759, t40761, t40765, t40771, t40782, t40784, t40789, t40792, t40801, t40804, t40810, t4362, t4364, t4366, t800);
        let (t40816, t40822, t40824, t40836, t40838, t40840) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1387(t10766, t10811, t10788, t14923, t10799, t10759, t2735, t40628, t854, t10890, t2707, t10896, t2703);
    (t40763, t40769, t40811, t40816, t40822, t40824, t40836, t40838, t40840)
}
