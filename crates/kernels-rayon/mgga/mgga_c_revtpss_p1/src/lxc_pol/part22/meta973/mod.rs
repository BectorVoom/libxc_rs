//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta973 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3260;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3261;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3262;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3263;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3264;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3265;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3266;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta973(t18413: f64, t18525: f64, t2661: f64, t40693: f64, t10726: f64, t4366: f64, t2723: f64, t61647: f64, t10886: f64, t18608: f64, t808: f64, t2394: f64, t2721: f64, t40462: f64, t40625: f64, t40630: f64, t40638: f64, t40639: f64, t40645: f64, t40654: f64, t5966: f64, t827: f64, t828: f64, t851: f64, t18352: f64, t2710: f64, t2713: f64, t10722: f64, t6030: f64, t18419: f64, t9775: f64, t14791: f64, t14802: f64, t40679: f64, t40681: f64, t40691: f64, t40707: f64, t40711: f64, t40722: f64, t4362: f64, t50703: f64, t50706: f64, t6022: f64, t10777: f64, t18481: f64, t50945: f64, t18333: f64, t51123: f64, t18349: f64, t2689: f64, t14494: f64, t14785: f64, t14894: f64, t2745: f64, t36833: f64, t40732: f64, t4424: f64, t4433: f64, t50423: f64, t50474: f64, t50722: f64, t50724: f64, t50728: f64, t50732: f64, t14923: f64, t18521: f64, t10770: f64, t10943: f64, t18426: f64, t18469: f64, t18627: f64, t2646: f64, t2747: f64, t4364: f64, t50736: f64, t50740: f64, t50744: f64, t50748: f64, t50752: f64, t50754: f64, t124: f64, t5977: f64, t10779: f64, t2749: f64, t14686: f64, t14931: f64, t2662: f64, t61625: f64, t18599: f64, t837: f64, t10760: f64, t18409: f64, t9794: f64, t5984: f64, t14749: f64, t40673: f64, t40737: f64, t4450: f64, t50771: f64, t50773: f64, t50784: f64, t50957: f64, t235: f64, t239: f64, t2476: f64, t246: f64, t4365: f64, t14917: f64, t18444: f64, t23334: f64, t40753: f64, t40759: f64, t40761: f64, t40765: f64, t40771: f64, t4504: f64, t50791: f64, t50933: f64, t50937: f64, t50939: f64, t50941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61866, t61879) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3260(t18413, t18525, t2661, t40693, t10726, t4366, t2723, t61647, t10886, t18608, t808, t2394, t2721, t40462, t40625, t40630, t40638, t40639, t40645, t40654, t5966, t827, t828, t851);
        let t61899 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3261(t18352, t2710, t2713, t10722, t6030, t18419, t9775, t14791, t14802, t40679, t40681, t40691, t40707, t40711, t40722, t4362, t50703, t50706, t6022);
        let t61929 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3262(t10777, t18481, t50945, t18333, t51123, t18349, t2689, t14494, t14785, t14791, t14894, t2745, t36833, t40732, t4424, t4433, t50423, t50474, t50722, t50724, t50728, t50732);
        let t61954 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3263(t14923, t18521, t10770, t10943, t18426, t18469, t18627, t2646, t2745, t2747, t4362, t4364, t50736, t50740, t50744, t50748, t50752, t50754);
        let (t61956, t61959, t61969, t61973, t61977) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3264(t124, t5977, t10777, t10779, t2749, t14686, t14931, t4366, t2661, t2662, t61625, t18599, t837);
        let t61987 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3265(t10760, t18409, t9794, t10777, t10779, t5984, t837, t14749, t40673, t40737, t4450, t50771, t50773, t50784, t50957, t61959, t61969, t61973, t61977);
        let (t61999, t62000, t62002, t62008) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3266(t235, t239, t2476, t246, t4365, t10770, t14802, t14917, t18444, t23334, t2745, t40753, t40759, t40761, t40765, t40771, t4504, t50791, t50933, t50937, t50939, t50941);
    (t61866, t61879, t61899, t61929, t61954, t61956, t61987, t61999, t62000, t62002, t62008)
}
