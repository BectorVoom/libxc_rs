//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta756 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2651;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2652;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2653;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta756(t48796: f64, t47247: f64, t828: f64, t13967: f64, t9962: f64, t13941: f64, t46740: f64, t221: f64, t47273: f64, t13785: f64, t9816: f64, t13770: f64, t9775: f64, t40690: f64, t5610: f64, t13783: f64, t13784: f64, t13789: f64, t13804: f64, t36776: f64, t3934: f64, t3938: f64, t46432: f64, t46861: f64, t46863: f64, t46865: f64, t48073: f64, t48105: f64, t48113: f64, t48786: f64, t48790: f64, t48792: f64, t48794: f64, t5671: f64, t9835: f64, t9956: f64, t5618: f64, t9784: f64, t820: f64, t844: f64, t9991: f64, t13807: f64, t13767: f64, t2661: f64, t3829: f64, t48347: f64, t13776: f64, t46644: f64, t5622: f64, t5614: f64, t9779: f64, t40488: f64, t13995: f64, t2659: f64, t4086: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48797, t48798, t48811, t48814, t48825, t48827) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2651(t48796, t47247, t828, t13967, t9962, t13941, t46740, t221, t47273, t13785, t9816, t13770, t9775);
        let t48832 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2652(t40690, t5610, t13783, t13784, t13789, t13804, t36776, t3934, t3938, t46432, t46861, t46863, t46865, t48073, t48105, t48113, t48786, t48790, t48792, t48794, t48797, t48798, t48811, t48814, t48825, t48827, t5671, t9835, t9956);
        let (t48833, t48837, t48845, t48847) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2653(t5618, t9784, t820, t844, t9991, t13807, t13767, t2661, t3829, t48347, t13776, t9775);
        let (t48848, t48849, t48851, t48853, t48855, t48862) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2654(t48847, t46644, t5622, t5614, t9779, t40488, t5610, t13995, t9962, t2659, t4086, t816);
    (t48832, t48833, t48837, t48845, t48848, t48849, t48851, t48853, t48855, t48862)
}
