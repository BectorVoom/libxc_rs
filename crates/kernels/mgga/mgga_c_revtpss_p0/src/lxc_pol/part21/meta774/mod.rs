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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2750;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2751;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2752;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2753;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2754;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2755;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2756;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta774<F: Float>(t14671: F, t14686: F, t14931: F, t2724: F, t10811: F, t14707: F, t14874: F, t14673: F, t40731: F, t40593: F, t4447: F, t4462: F, t10760: F, t40763: F, t4353: F, t1559: F, t775: F, t40834: F, t854: F, t14587: F, t2735: F, t40798: F, t826: F, t14547: F, t14676: F, t14894: F, t2745: F, t36833: F, t4364: F, t50560: F, t50573: F, t50577: F, t50579: F, t50582: F, t50586: F, t50590: F, t50594: F, t837: F, t10777: F, t10779: F, t2749: F, t50412: F, t4452: F, t2646: F, t4343: F, t836: F, t10638: F, t2723: F, t10943: F, t10627: F, t10861: F, t14691: F, t14785: F, t231: F, t2747: F, t40581: F, t40586: F, t40594: F, t40600: F, t40607: F, t40611: F, t40673: F, t4362: F, t4365: F, t50423: F, t14933: F, t2482: F, t2668: F, t2719: F, t2710: F, t4371: F, t9732: F, t10886: F, t14833: F, t808: F, t10900: F, t124: F, t14791: F, t2394: F, t40625: F, t40630: F, t40638: F, t40639: F, t40643: F, t40645: F, t40654: F, t40662: F, t40669: F, t40679: F, t40681: F, t40686: F, t4366: F, t4457: F, t50151: F, t50418: F, t799: F, t800: F, t14793: F, t14774: F, t2652: F, t10726: F, t14860: F, t2661: F, t2662: F, t4352: F, t14652: F, t4416: F, t14663: F, t221: F, t2484: F, t2485: F, t40691: F, t40696: F, t40700: F, t40705: F, t40707: F, t40711: F, t40719: F, t40722: F, t40728: F, t40732: F, t14919: F, t14904: F, t14923: F, t241: F, t40322: F, t820: F, t10665: F, t40325: F, t2659: F, t2783: F, t816: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50598, t50600, t50602, t50605, t50607, t50608) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2750::<F>(t14671, t14686, t14931, t2724, t10811, t14707, t14874, t14673, t40731, t40593, t4447, t4462);
        let (t50613, t50621) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2751::<F>(t10760, t40763, t4353, t1559, t775, t40834, t854, t14587, t2735, t40798, t826, t14547, t14676, t14894, t2745, t36833, t4364, t50560, t50573, t50577, t50579, t50582, t50586, t50590, t50594, t50598, t50600, t50602, t50605, t50607, t50608, t837);
        let (t50628, t50632, t50634, t50643, t50649) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2752::<F>(t10777, t10779, t2749, t50412, t14686, t837, t40593, t4452, t14671, t2646, t4343, t836);
        let (t50666, t50675) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2753::<F>(t10638, t2723, t10943, t14671, t14686, t14931, t10627, t10861, t14676, t14691, t14785, t231, t2646, t2745, t2747, t2749, t40581, t40586, t40594, t40600, t40607, t40611, t40673, t4362, t4364, t4365, t50423, t50628, t50632, t50634, t50643, t50649);
        let t50711 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2754::<F>(t14933, t2482, t2668, t2719, t2710, t4371, t9732, t10886, t14833, t808, t10900, t124, t14791, t2394, t40625, t40630, t40638, t40639, t40643, t40645, t40654, t40662, t40669, t40679, t40681, t40686, t4362, t4366, t4457, t50151, t50418, t799, t800);
        let (t50722, t50724, t50728, t50732, t50736) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2755::<F>(t10811, t14793, t14774, t2652, t10726, t14860, t2661, t4366, t2662, t837, t2646, t4352);
        let t50750 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2756::<F>(t14652, t2661, t2662, t837, t2646, t4416, t14663, t221, t2484, t2485, t40691, t40696, t40700, t40705, t40707, t40711, t40719, t40722, t40728, t40732, t50722, t50724, t50728, t50732, t50736);
        let (t50752, t50754, t50757, t50758, t50768) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2757::<F>(t10811, t14919, t14904, t14923, t241, t40322, t820, t10665, t40325, t2659, t2783, t816);
    (t50613, t50621, t50666, t50675, t50711, t50750, t50752, t50754, t50757, t50758, t50768)
}
