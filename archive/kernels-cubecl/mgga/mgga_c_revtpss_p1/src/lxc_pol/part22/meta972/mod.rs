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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3252;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3253;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3254;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3255;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3256;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3257;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3258;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3259;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta972<F: Float>(t10777: F, t40725: F, t5988: F, t837: F, t40593: F, t6037: F, t125: F, t18392: F, t124: F, t6016: F, t14686: F, t14931: F, t4366: F, t18498: F, t221: F, t10703: F, t2674: F, t14468: F, t1544: F, t231: F, t2477: F, t2745: F, t2747: F, t4365: F, t50436: F, t50443: F, t50453: F, t50457: F, t50466: F, t828: F, t851: F, t14663: F, t40455: F, t40473: F, t40475: F, t40477: F, t40489: F, t4364: F, t50472: F, t50493: F, t50497: F, t50502: F, t50504: F, t836: F, t10811: F, t18482: F, t5977: F, t14749: F, t14785: F, t14791: F, t1559: F, t2749: F, t50518: F, t50522: F, t50524: F, t50526: F, t50529: F, t50531: F, t50540: F, t18462: F, t18466: F, t14872: F, t18426: F, t40507: F, t40509: F, t40518: F, t40523: F, t40526: F, t40529: F, t40532: F, t40535: F, t18615: F, t10744: F, t18418: F, t808: F, t10900: F, t18627: F, t2394: F, t2724: F, t4362: F, t50573: F, t50577: F, t50579: F, t50581: F, t50586: F, t50590: F, t50594: F, t50598: F, t5984: F, t800: F, t18446: F, t50600: F, t50602: F, t50604: F, t50606: F, t50608: F, t50611: F, t50615: F, t50619: F, t50628: F, t50632: F, t10886: F, t18599: F, t40834: F, t854: F, t10770: F, t14676: F, t18637: F, t2723: F, t40594: F, t40600: F, t40607: F, t40611: F, t50634: F, t50643: F, t50673: F, t50681: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t61697, t61699, t61701, t61715, t61718) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3252::<F>(t10777, t40725, t5988, t837, t40593, t6037, t125, t18392, t124, t6016, t14686, t14931, t4366);
        let t61730 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3253::<F>(t18498, t221, t10703, t2674, t14468, t1544, t231, t2477, t2745, t2747, t4365, t50436, t50443, t50453, t50457, t50466, t61697, t61699, t61701, t61718, t828, t837, t851);
        let t61748 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3254::<F>(t14663, t2745, t40455, t40473, t40475, t40477, t40489, t4364, t4365, t50472, t50493, t50497, t50502, t50504);
        let (t61749, t61756, t61772) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3255::<F>(t6016, t836, t10811, t18482, t5977, t14749, t14785, t14791, t1559, t2745, t2749, t50518, t50522, t50524, t50526, t50529, t50531, t50540);
        let t61789 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3256::<F>(t10811, t18462, t18466, t14872, t18426, t2745, t2747, t40507, t40509, t40518, t40523, t40526, t40529, t40532, t40535);
        let (t61791, t61814) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3257::<F>(t125, t18615, t10744, t18418, t808, t10900, t18627, t2394, t2724, t2747, t4362, t4364, t4366, t50573, t50577, t50579, t50581, t50586, t50590, t50594, t50598, t5984, t800);
        let t61829 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3258::<F>(t10811, t18446, t50600, t50602, t50604, t50606, t50608, t50611, t50615, t50619, t50628, t50632);
        let (t61837, t61852) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3259::<F>(t10886, t18599, t808, t1544, t1559, t40834, t854, t10770, t14676, t18426, t18637, t2394, t2723, t2745, t2747, t40594, t40600, t40607, t40611, t4362, t50634, t50643, t50673, t50681);
    (t61715, t61730, t61748, t61749, t61756, t61772, t61789, t61791, t61814, t61829, t61837, t61852)
}
