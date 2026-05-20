//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta756 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2651;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2652;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2653;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta756<F: Float>(t48796: F, t47247: F, t828: F, t13967: F, t9962: F, t13941: F, t46740: F, t221: F, t47273: F, t13785: F, t9816: F, t13770: F, t9775: F, t40690: F, t5610: F, t13783: F, t13784: F, t13789: F, t13804: F, t36776: F, t3934: F, t3938: F, t46432: F, t46861: F, t46863: F, t46865: F, t48073: F, t48105: F, t48113: F, t48786: F, t48790: F, t48792: F, t48794: F, t5671: F, t9835: F, t9956: F, t5618: F, t9784: F, t820: F, t844: F, t9991: F, t13807: F, t13767: F, t2661: F, t3829: F, t48347: F, t13776: F, t46644: F, t5622: F, t5614: F, t9779: F, t40488: F, t13995: F, t2659: F, t4086: F, t816: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t48797, t48798, t48811, t48814, t48825, t48827) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2651::<F>(t48796, t47247, t828, t13967, t9962, t13941, t46740, t221, t47273, t13785, t9816, t13770, t9775);
        let t48832 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2652::<F>(t40690, t5610, t13783, t13784, t13789, t13804, t36776, t3934, t3938, t46432, t46861, t46863, t46865, t48073, t48105, t48113, t48786, t48790, t48792, t48794, t48797, t48798, t48811, t48814, t48825, t48827, t5671, t9835, t9956);
        let (t48833, t48837, t48845, t48847) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2653::<F>(t5618, t9784, t820, t844, t9991, t13807, t13767, t2661, t3829, t48347, t13776, t9775);
        let (t48848, t48849, t48851, t48853, t48855, t48862) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2654::<F>(t48847, t46644, t5622, t5614, t9779, t40488, t5610, t13995, t9962, t2659, t4086, t816);
    (t48832, t48833, t48837, t48845, t48848, t48849, t48851, t48853, t48855, t48862)
}
