//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta890 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2831;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2832;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2833;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2834;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2835;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2836;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2837;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2838;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2839;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta890<F: Float>(t10811: F, t23297: F, t14923: F, t23336: F, t14894: F, t18525: F, t40455: F, t40489: F, t4364: F, t50436: F, t50443: F, t50454: F, t50457: F, t50505: F, t50524: F, t61689: F, t61692: F, t61697: F, t61699: F, t61718: F, t61727: F, t61754: F, t76284: F, t10900: F, t14586: F, t14785: F, t14791: F, t1544: F, t1558: F, t18393: F, t23160: F, t23262: F, t2730: F, t2745: F, t36833: F, t40507: F, t40518: F, t40535: F, t40868: F, t4343: F, t4362: F, t4366: F, t50532: F, t50582: F, t50605: F, t5984: F, t5988: F, t6035: F, t61749: F, t61756: F, t61774: F, t61776: F, t61797: F, t61817: F, t76474: F, t775: F, t800: F, t23167: F, t243: F, t10726: F, t2661: F, t2723: F, t18408: F, t23334: F, t61625: F, t10850: F, t221: F, t23172: F, t2485: F, t23281: F, t2652: F, t23148: F, t2477: F, t40607: F, t40611: F, t4433: F, t50607: F, t50608: F, t50611: F, t50615: F, t50619: F, t50634: F, t50681: F, t6017: F, t61833: F, t61839: F, t828: F, t851: F, t10858: F, t23257: F, t23279: F, t10703: F, t2674: F, t2662: F, t61579: F, t10698: F, t18392: F, t40625: F, t40638: F, t40639: F, t40654: F, t40691: F, t40711: F, t50446: F, t50703: F, t50707: F, t5962: F, t5966: F, t61860: F, t61864: F, t61877: F, t1559: F, t18608: F, t23253: F, t40348: F, t10777: F, t10779: F, t10786: F, t18426: F, t18435: F, t18627: F, t18632: F, t2747: F, t40673: F, t40722: F, t4424: F, t4450: F, t50774: F, t50957: F, t61701: F, t61888: F, t61890: F, t61892: F, t61913: F, t61916: F, t61924: F, t61952: F, t61959: F, t10905: F, t23275: F, t61956: F, t40725: F, t23301: F, t125: F, t23114: F, t61715: F, t10871: F, t4423: F, t14931: F, t23331: F, t10770: F, t14676: F, t18444: F, t18469: F, t18637: F, t40664: F, t40737: F, t61791: F, t837: F, t4352: F, t23285: F, t2741: F, t14494: F, t23266: F, t40759: F, t40765: F, t40771: F, t50939: F, t50941: F, t61969: F, t61973: F, t61977: F, t61981: F, t61985: F, t62012: F, t62015: F, t76242: F, t76372: F, t23289: F, t124: F, t40782: F, t50943: F, t50955: F, t50978: F, t62021: F, t62029: F, t62033: F, t62045: F, t62056: F, t62058: F, t62069: F, t62072: F, t62089: F, t62095: F, t62105: F, t76421: F, t799: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t76517 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2831::<F>(t10811, t23297, t14923, t23336, t14894, t18525, t40455, t40489, t4364, t50436, t50443, t50454, t50457, t50505, t50524, t61689, t61692, t61697, t61699, t61718, t61727, t61754, t76284);
        let t76557 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2832::<F>(t10900, t14586, t14785, t14791, t1544, t1558, t18393, t23160, t23262, t2730, t2745, t36833, t40507, t40518, t40535, t40868, t4343, t4362, t4366, t50532, t50582, t50605, t5984, t5988, t6035, t61749, t61756, t61774, t61776, t61797, t61817, t76474, t775, t800);
        let (t76569, t76572, t76583, t76587, t76591) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2833::<F>(t23167, t243, t10726, t2661, t2723, t14586, t18408, t23334, t61625, t10850, t221, t23172, t2485);
        let t76595 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2834::<F>(t23281, t2652, t14785, t23148, t2477, t2745, t40607, t40611, t4433, t50607, t50608, t50611, t50615, t50619, t50634, t50681, t6017, t61833, t61839, t76572, t76583, t76587, t76591, t775, t828, t851);
        let t76633 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2835::<F>(t10858, t23257, t221, t23279, t10703, t2674, t2661, t2662, t6035, t61579, t10698, t1544, t18392, t2477, t40625, t40638, t40639, t40654, t40691, t40711, t4343, t50446, t50703, t50707, t5962, t5966, t61860, t61864, t61877, t775, t828, t851);
        let t76676 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2836::<F>(t1559, t18608, t2661, t2662, t23253, t40348, t10777, t10779, t5984, t10786, t18426, t18435, t18627, t18632, t2745, t2747, t40673, t40722, t4362, t4364, t4424, t4450, t50774, t50957, t61701, t61888, t61890, t61892, t61913, t61916, t61924, t61952, t61959, t76284);
        let (t76677, t76689, t76701, t76703, t76705) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2837::<F>(t10905, t23275, t10777, t10779, t6035, t61956, t1559, t40725, t5988, t14923, t23301, t125, t23114);
        let (t76726, t76742) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2838::<F>(t10777, t10779, t6035, t61715, t10871, t4423, t14931, t23334, t61956, t10811, t23331, t10770, t14676, t14894, t18426, t18444, t18469, t18637, t2723, t2745, t2747, t40664, t40673, t40737, t4343, t4362, t4364, t4366, t4424, t6017, t61791, t76284, t76677, t76689, t76701, t76703, t76705, t837);
        let t76776 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2839::<F>(t2661, t2662, t4352, t6017, t23285, t2741, t14494, t14785, t14791, t14894, t1559, t23266, t2730, t2745, t36833, t40759, t40765, t40771, t50939, t50941, t61969, t61973, t61977, t61981, t61985, t62012, t62015, t76242, t76372, t76474, t775, t800, t837);
        let t76800 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2840::<F>(t23289, t2741, t2661, t2662, t6035, t61625, t124, t40782, t50943, t50955, t50978, t62021, t62029, t62033, t62045, t62056, t62058, t62069, t62072, t62089, t62095, t62105, t76421, t799, t800);
    (t76517, t76557, t76569, t76595, t76633, t76676, t76726, t76742, t76776, t76800)
}
