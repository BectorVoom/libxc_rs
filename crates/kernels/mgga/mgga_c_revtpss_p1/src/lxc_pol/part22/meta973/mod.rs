//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta973 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3260;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3261;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3262;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3263;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3264;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3265;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3266;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta973<F: Float>(t18413: F, t18525: F, t2661: F, t40693: F, t10726: F, t4366: F, t2723: F, t61647: F, t10886: F, t18608: F, t808: F, t2394: F, t2721: F, t40462: F, t40625: F, t40630: F, t40638: F, t40639: F, t40645: F, t40654: F, t5966: F, t827: F, t828: F, t851: F, t18352: F, t2710: F, t2713: F, t10722: F, t6030: F, t18419: F, t9775: F, t14791: F, t14802: F, t40679: F, t40681: F, t40691: F, t40707: F, t40711: F, t40722: F, t4362: F, t50703: F, t50706: F, t6022: F, t10777: F, t18481: F, t50945: F, t18333: F, t51123: F, t18349: F, t2689: F, t14494: F, t14785: F, t14894: F, t2745: F, t36833: F, t40732: F, t4424: F, t4433: F, t50423: F, t50474: F, t50722: F, t50724: F, t50728: F, t50732: F, t14923: F, t18521: F, t10770: F, t10943: F, t18426: F, t18469: F, t18627: F, t2646: F, t2747: F, t4364: F, t50736: F, t50740: F, t50744: F, t50748: F, t50752: F, t50754: F, t124: F, t5977: F, t10779: F, t2749: F, t14686: F, t14931: F, t2662: F, t61625: F, t18599: F, t837: F, t10760: F, t18409: F, t9794: F, t5984: F, t14749: F, t40673: F, t40737: F, t4450: F, t50771: F, t50773: F, t50784: F, t50957: F, t235: F, t239: F, t2476: F, t246: F, t4365: F, t14917: F, t18444: F, t23334: F, t40753: F, t40759: F, t40761: F, t40765: F, t40771: F, t4504: F, t50791: F, t50933: F, t50937: F, t50939: F, t50941: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t61866, t61879) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3260::<F>(t18413, t18525, t2661, t40693, t10726, t4366, t2723, t61647, t10886, t18608, t808, t2394, t2721, t40462, t40625, t40630, t40638, t40639, t40645, t40654, t5966, t827, t828, t851);
        let t61899 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3261::<F>(t18352, t2710, t2713, t10722, t6030, t18419, t9775, t14791, t14802, t40679, t40681, t40691, t40707, t40711, t40722, t4362, t50703, t50706, t6022);
        let t61929 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3262::<F>(t10777, t18481, t50945, t18333, t51123, t18349, t2689, t14494, t14785, t14791, t14894, t2745, t36833, t40732, t4424, t4433, t50423, t50474, t50722, t50724, t50728, t50732);
        let t61954 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3263::<F>(t14923, t18521, t10770, t10943, t18426, t18469, t18627, t2646, t2745, t2747, t4362, t4364, t50736, t50740, t50744, t50748, t50752, t50754);
        let (t61956, t61959, t61969, t61973, t61977) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3264::<F>(t124, t5977, t10777, t10779, t2749, t14686, t14931, t4366, t2661, t2662, t61625, t18599, t837);
        let t61987 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3265::<F>(t10760, t18409, t9794, t10777, t10779, t5984, t837, t14749, t40673, t40737, t4450, t50771, t50773, t50784, t50957, t61959, t61969, t61973, t61977);
        let (t61999, t62000, t62002, t62008) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3266::<F>(t235, t239, t2476, t246, t4365, t10770, t14802, t14917, t18444, t23334, t2745, t40753, t40759, t40761, t40765, t40771, t4504, t50791, t50933, t50937, t50939, t50941);
    (t61866, t61879, t61899, t61929, t61954, t61956, t61987, t61999, t62000, t62002, t62008)
}
