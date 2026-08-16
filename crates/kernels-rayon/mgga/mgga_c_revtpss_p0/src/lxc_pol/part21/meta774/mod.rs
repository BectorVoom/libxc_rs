//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta774 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2750;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2751;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2752;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2753;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2754;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2755;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2756;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta774(t14671: f64, t14686: f64, t14931: f64, t2724: f64, t10811: f64, t14707: f64, t14874: f64, t14673: f64, t40731: f64, t40593: f64, t4447: f64, t4462: f64, t10760: f64, t40763: f64, t4353: f64, t1559: f64, t775: f64, t40834: f64, t854: f64, t14587: f64, t2735: f64, t40798: f64, t826: f64, t14547: f64, t14676: f64, t14894: f64, t2745: f64, t36833: f64, t4364: f64, t50560: f64, t50573: f64, t50577: f64, t50579: f64, t50582: f64, t50586: f64, t50590: f64, t50594: f64, t837: f64, t10777: f64, t10779: f64, t2749: f64, t50412: f64, t4452: f64, t2646: f64, t4343: f64, t836: f64, t10638: f64, t2723: f64, t10943: f64, t10627: f64, t10861: f64, t14691: f64, t14785: f64, t231: f64, t2747: f64, t40581: f64, t40586: f64, t40594: f64, t40600: f64, t40607: f64, t40611: f64, t40673: f64, t4362: f64, t4365: f64, t50423: f64, t14933: f64, t2482: f64, t2668: f64, t2719: f64, t2710: f64, t4371: f64, t9732: f64, t10886: f64, t14833: f64, t808: f64, t10900: f64, t124: f64, t14791: f64, t2394: f64, t40625: f64, t40630: f64, t40638: f64, t40639: f64, t40643: f64, t40645: f64, t40654: f64, t40662: f64, t40669: f64, t40679: f64, t40681: f64, t40686: f64, t4366: f64, t4457: f64, t50151: f64, t50418: f64, t799: f64, t800: f64, t14793: f64, t14774: f64, t2652: f64, t10726: f64, t14860: f64, t2661: f64, t2662: f64, t4352: f64, t14652: f64, t4416: f64, t14663: f64, t221: f64, t2484: f64, t2485: f64, t40691: f64, t40696: f64, t40700: f64, t40705: f64, t40707: f64, t40711: f64, t40719: f64, t40722: f64, t40728: f64, t40732: f64, t14919: f64, t14904: f64, t14923: f64, t241: f64, t40322: f64, t820: f64, t10665: f64, t40325: f64, t2659: f64, t2783: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50598, t50600, t50602, t50605, t50607, t50608) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2750(t14671, t14686, t14931, t2724, t10811, t14707, t14874, t14673, t40731, t40593, t4447, t4462);
        let (t50613, t50621) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2751(t10760, t40763, t4353, t1559, t775, t40834, t854, t14587, t2735, t40798, t826, t14547, t14676, t14894, t2745, t36833, t4364, t50560, t50573, t50577, t50579, t50582, t50586, t50590, t50594, t50598, t50600, t50602, t50605, t50607, t50608, t837);
        let (t50628, t50632, t50634, t50643, t50649) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2752(t10777, t10779, t2749, t50412, t14686, t837, t40593, t4452, t14671, t2646, t4343, t836);
        let (t50666, t50675) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2753(t10638, t2723, t10943, t14671, t14686, t14931, t10627, t10861, t14676, t14691, t14785, t231, t2646, t2745, t2747, t2749, t40581, t40586, t40594, t40600, t40607, t40611, t40673, t4362, t4364, t4365, t50423, t50628, t50632, t50634, t50643, t50649);
        let t50711 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2754(t14933, t2482, t2668, t2719, t2710, t4371, t9732, t10886, t14833, t808, t10900, t124, t14791, t2394, t40625, t40630, t40638, t40639, t40643, t40645, t40654, t40662, t40669, t40679, t40681, t40686, t4362, t4366, t4457, t50151, t50418, t799, t800);
        let (t50722, t50724, t50728, t50732, t50736) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2755(t10811, t14793, t14774, t2652, t10726, t14860, t2661, t4366, t2662, t837, t2646, t4352);
        let t50750 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2756(t14652, t2661, t2662, t837, t2646, t4416, t14663, t221, t2484, t2485, t40691, t40696, t40700, t40705, t40707, t40711, t40719, t40722, t40728, t40732, t50722, t50724, t50728, t50732, t50736);
        let (t50752, t50754, t50757, t50758, t50768) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2757(t10811, t14919, t14904, t14923, t241, t40322, t820, t10665, t40325, t2659, t2783, t816);
    (t50613, t50621, t50666, t50675, t50711, t50750, t50752, t50754, t50757, t50758, t50768)
}
