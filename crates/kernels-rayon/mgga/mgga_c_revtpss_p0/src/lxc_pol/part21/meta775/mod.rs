//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta775 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2758;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2759;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2760;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2761;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2762;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2763;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2764;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta775(t808: f64, t853: f64, t14792: f64, t50768: f64, t14688: f64, t40731: f64, t10777: f64, t14671: f64, t14686: f64, t2754: f64, t14749: f64, t221: f64, t10703: f64, t2674: f64, t10666: f64, t2745: f64, t2747: f64, t2749: f64, t40737: f64, t40744: f64, t40748: f64, t40750: f64, t40753: f64, t40759: f64, t40761: f64, t40765: f64, t40771: f64, t4364: f64, t4365: f64, t50459: f64, t50752: f64, t50754: f64, t50757: f64, t50758: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64, t39442: f64, t49865: f64, t49867: f64, t49868: f64, t49869: f64, t49870: f64, t49872: f64, t49873: f64, t49877: f64, t49879: f64, t49882: f64, t49885: f64, t49892: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39764: f64, t49898: f64, t49912: f64, t49913: f64, t39770: f64, t39773: f64, t49918: f64, t49920: f64, t49925: f64, t49927: f64, t49930: f64, t49941: f64, t49944: f64, t49945: f64, t49956: f64, t49958: f64, t49959: f64, t49964: f64, t49967: f64, t49969: f64, t49971: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t49979: f64, t49982: f64, t49984: f64, t49987: f64, t49992: f64, t49994: f64, t49995: f64, t50037: f64, t40084: f64, t40088: f64, t40099: f64, t40103: f64, t40115: f64, t40131: f64, t50038: f64, t50039: f64, t50045: f64, t50046: f64, t50048: f64, t50051: f64, t50055: f64, t50056: f64, t50059: f64, t50063: f64, t50064: f64, t4398: f64, t9323: f64, t39989: f64, t40137: f64, t40141: f64, t50065: f64, t50070: f64, t50085: f64, t50091: f64, t50093: f64, t50095: f64, t50096: f64, t50098: f64, t50100: f64, t50101: f64, t50106: f64, t50114: f64, t50115: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50769, t50771, t50774, t50784, t50789) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2758(t808, t853, t14792, t50768, t14688, t40731, t10777, t14671, t14686, t2754, t14749, t221);
        let t50793 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2759(t10703, t2674, t50789, t10666, t2745, t2747, t2749, t40737, t40744, t40748, t40750, t40753, t40759, t40761, t40765, t40771, t4364, t4365, t50459, t50752, t50754, t50757, t50758, t50771, t50774, t50784);
        let t50844 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2760(t39419, t39422, t39429, t39432, t39442, t49865, t49867, t49868, t49869, t49870, t49872, t49873, t49877, t49879, t49882, t49885, t49892);
        let t50845 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2761(t39483, t39520, t39528, t39531, t39534, t39537, t39540, t39741, t39744, t39747, t39750, t39756, t39760, t39764, t49898, t49912, t49913);
        let t50847 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2762(t39770, t39773, t49918, t49920, t49925, t49927, t49930, t49941, t49944, t49945, t49956, t49958, t49959, t49964, t49967, t49969, t49971);
        let t50848 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2763(t39783, t39786, t39791, t39795, t39799, t39807, t39813, t39818, t39823, t49979, t49982, t49984, t49987, t49992, t49994, t49995, t50037);
        let t50851 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2764(t40084, t40088, t40099, t40103, t40115, t40131, t50038, t50039, t50045, t50046, t50048, t50051, t50055, t50056, t50059, t50063, t50064);
        let (t50853, t50854) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2765(t4398, t9323, t39989, t40137, t40141, t50065, t50070, t50085, t50091, t50093, t50095, t50096, t50098, t50100, t50101, t50106, t50114, t50115);
    (t50769, t50793, t50844, t50845, t50847, t50848, t50851, t50853, t50854)
}
