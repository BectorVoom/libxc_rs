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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2758;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2759;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2760;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2761;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2762;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2763;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2764;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta775<F: Float>(t808: F, t853: F, t14792: F, t50768: F, t14688: F, t40731: F, t10777: F, t14671: F, t14686: F, t2754: F, t14749: F, t221: F, t10703: F, t2674: F, t10666: F, t2745: F, t2747: F, t2749: F, t40737: F, t40744: F, t40748: F, t40750: F, t40753: F, t40759: F, t40761: F, t40765: F, t40771: F, t4364: F, t4365: F, t50459: F, t50752: F, t50754: F, t50757: F, t50758: F, t39419: F, t39422: F, t39429: F, t39432: F, t39442: F, t49865: F, t49867: F, t49868: F, t49869: F, t49870: F, t49872: F, t49873: F, t49877: F, t49879: F, t49882: F, t49885: F, t49892: F, t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t39760: F, t39764: F, t49898: F, t49912: F, t49913: F, t39770: F, t39773: F, t49918: F, t49920: F, t49925: F, t49927: F, t49930: F, t49941: F, t49944: F, t49945: F, t49956: F, t49958: F, t49959: F, t49964: F, t49967: F, t49969: F, t49971: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t49979: F, t49982: F, t49984: F, t49987: F, t49992: F, t49994: F, t49995: F, t50037: F, t40084: F, t40088: F, t40099: F, t40103: F, t40115: F, t40131: F, t50038: F, t50039: F, t50045: F, t50046: F, t50048: F, t50051: F, t50055: F, t50056: F, t50059: F, t50063: F, t50064: F, t4398: F, t9323: F, t39989: F, t40137: F, t40141: F, t50065: F, t50070: F, t50085: F, t50091: F, t50093: F, t50095: F, t50096: F, t50098: F, t50100: F, t50101: F, t50106: F, t50114: F, t50115: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50769, t50771, t50774, t50784, t50789) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2758::<F>(t808, t853, t14792, t50768, t14688, t40731, t10777, t14671, t14686, t2754, t14749, t221);
        let t50793 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2759::<F>(t10703, t2674, t50789, t10666, t2745, t2747, t2749, t40737, t40744, t40748, t40750, t40753, t40759, t40761, t40765, t40771, t4364, t4365, t50459, t50752, t50754, t50757, t50758, t50771, t50774, t50784);
        let t50844 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2760::<F>(t39419, t39422, t39429, t39432, t39442, t49865, t49867, t49868, t49869, t49870, t49872, t49873, t49877, t49879, t49882, t49885, t49892);
        let t50845 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2761::<F>(t39483, t39520, t39528, t39531, t39534, t39537, t39540, t39741, t39744, t39747, t39750, t39756, t39760, t39764, t49898, t49912, t49913);
        let t50847 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2762::<F>(t39770, t39773, t49918, t49920, t49925, t49927, t49930, t49941, t49944, t49945, t49956, t49958, t49959, t49964, t49967, t49969, t49971);
        let t50848 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2763::<F>(t39783, t39786, t39791, t39795, t39799, t39807, t39813, t39818, t39823, t49979, t49982, t49984, t49987, t49992, t49994, t49995, t50037);
        let t50851 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2764::<F>(t40084, t40088, t40099, t40103, t40115, t40131, t50038, t50039, t50045, t50046, t50048, t50051, t50055, t50056, t50059, t50063, t50064);
        let (t50853, t50854) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2765::<F>(t4398, t9323, t39989, t40137, t40141, t50065, t50070, t50085, t50091, t50093, t50095, t50096, t50098, t50100, t50101, t50106, t50114, t50115);
    (t50769, t50793, t50844, t50845, t50847, t50848, t50851, t50853, t50854)
}
