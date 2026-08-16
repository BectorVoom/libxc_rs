//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta972 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3252;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3253;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3254;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3255;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3256;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3257;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3258;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3259;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta972(t10777: f64, t40725: f64, t5988: f64, t837: f64, t40593: f64, t6037: f64, t125: f64, t18392: f64, t124: f64, t6016: f64, t14686: f64, t14931: f64, t4366: f64, t18498: f64, t221: f64, t10703: f64, t2674: f64, t14468: f64, t1544: f64, t231: f64, t2477: f64, t2745: f64, t2747: f64, t4365: f64, t50436: f64, t50443: f64, t50453: f64, t50457: f64, t50466: f64, t828: f64, t851: f64, t14663: f64, t40455: f64, t40473: f64, t40475: f64, t40477: f64, t40489: f64, t4364: f64, t50472: f64, t50493: f64, t50497: f64, t50502: f64, t50504: f64, t836: f64, t10811: f64, t18482: f64, t5977: f64, t14749: f64, t14785: f64, t14791: f64, t1559: f64, t2749: f64, t50518: f64, t50522: f64, t50524: f64, t50526: f64, t50529: f64, t50531: f64, t50540: f64, t18462: f64, t18466: f64, t14872: f64, t18426: f64, t40507: f64, t40509: f64, t40518: f64, t40523: f64, t40526: f64, t40529: f64, t40532: f64, t40535: f64, t18615: f64, t10744: f64, t18418: f64, t808: f64, t10900: f64, t18627: f64, t2394: f64, t2724: f64, t4362: f64, t50573: f64, t50577: f64, t50579: f64, t50581: f64, t50586: f64, t50590: f64, t50594: f64, t50598: f64, t5984: f64, t800: f64, t18446: f64, t50600: f64, t50602: f64, t50604: f64, t50606: f64, t50608: f64, t50611: f64, t50615: f64, t50619: f64, t50628: f64, t50632: f64, t10886: f64, t18599: f64, t40834: f64, t854: f64, t10770: f64, t14676: f64, t18637: f64, t2723: f64, t40594: f64, t40600: f64, t40607: f64, t40611: f64, t50634: f64, t50643: f64, t50673: f64, t50681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61697, t61699, t61701, t61715, t61718) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3252(t10777, t40725, t5988, t837, t40593, t6037, t125, t18392, t124, t6016, t14686, t14931, t4366);
        let t61730 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3253(t18498, t221, t10703, t2674, t14468, t1544, t231, t2477, t2745, t2747, t4365, t50436, t50443, t50453, t50457, t50466, t61697, t61699, t61701, t61718, t828, t837, t851);
        let t61748 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3254(t14663, t2745, t40455, t40473, t40475, t40477, t40489, t4364, t4365, t50472, t50493, t50497, t50502, t50504);
        let (t61749, t61756, t61772) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3255(t6016, t836, t10811, t18482, t5977, t14749, t14785, t14791, t1559, t2745, t2749, t50518, t50522, t50524, t50526, t50529, t50531, t50540);
        let t61789 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3256(t10811, t18462, t18466, t14872, t18426, t2745, t2747, t40507, t40509, t40518, t40523, t40526, t40529, t40532, t40535);
        let (t61791, t61814) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3257(t125, t18615, t10744, t18418, t808, t10900, t18627, t2394, t2724, t2747, t4362, t4364, t4366, t50573, t50577, t50579, t50581, t50586, t50590, t50594, t50598, t5984, t800);
        let t61829 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3258(t10811, t18446, t50600, t50602, t50604, t50606, t50608, t50611, t50615, t50619, t50628, t50632);
        let (t61837, t61852) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3259(t10886, t18599, t808, t1544, t1559, t40834, t854, t10770, t14676, t18426, t18637, t2394, t2723, t2745, t2747, t40594, t40600, t40607, t40611, t4362, t50634, t50643, t50673, t50681);
    (t61715, t61730, t61748, t61749, t61756, t61772, t61789, t61791, t61814, t61829, t61837, t61852)
}
