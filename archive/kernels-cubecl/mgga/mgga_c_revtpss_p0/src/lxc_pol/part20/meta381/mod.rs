//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1383;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1384;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1385;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1386;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta381<F: Float>(t240: F, t9731: F, t10760: F, t2664: F, t10293: F, t124: F, t212: F, t800: F, t810: F, t820: F, t849: F, t9948: F, t857: F, t10722: F, t2479: F, t14832: F, t2430: F, t2475: F, t2661: F, t775: F, t2699: F, t2729: F, t2732: F, t235: F, t4503: F, t2453: F, t10728: F, t9794: F, t10886: F, t40236: F, t808: F, t123: F, t125: F, t2452: F, t40633: F, t10785: F, t10943: F, t14791: F, t2730: F, t40240: F, t40655: F, t40748: F, t40750: F, t40753: F, t40759: F, t40761: F, t4362: F, t4364: F, t4366: F, t10766: F, t10811: F, t10788: F, t14923: F, t10799: F, t10759: F, t2735: F, t40628: F, t854: F, t10890: F, t2707: F, t10896: F, t2703: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40763, t40765, t40769, t40771, t40781) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1383::<F>(t240, t9731, t10760, t2664, t10293, t124, t212, t800, t810, t820, t849, t9948);
        let (t40782, t40784, t40789, t40792) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1384::<F>(t40781, t857, t10722, t2479, t14832, t2430, t2475, t2661, t775, t2699, t2729, t2732);
        let (t40801, t40804, t40810) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1385::<F>(t235, t4503, t2453, t10728, t9794, t10886, t40236, t808, t123, t125, t2452, t40633, t810);
        let t40811 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1386::<F>(t10785, t10943, t124, t14791, t2730, t40240, t40655, t40748, t40750, t40753, t40759, t40761, t40765, t40771, t40782, t40784, t40789, t40792, t40801, t40804, t40810, t4362, t4364, t4366, t800);
        let (t40816, t40822, t40824, t40836, t40838, t40840) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1387::<F>(t10766, t10811, t10788, t14923, t10799, t10759, t2735, t40628, t854, t10890, t2707, t10896, t2703);
    (t40763, t40769, t40811, t40816, t40822, t40824, t40836, t40838, t40840)
}
