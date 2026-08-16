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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta890(t10811: f64, t23297: f64, t14923: f64, t23336: f64, t14894: f64, t18525: f64, t40455: f64, t40489: f64, t4364: f64, t50436: f64, t50443: f64, t50454: f64, t50457: f64, t50505: f64, t50524: f64, t61689: f64, t61692: f64, t61697: f64, t61699: f64, t61718: f64, t61727: f64, t61754: f64, t76284: f64, t10900: f64, t14586: f64, t14785: f64, t14791: f64, t1544: f64, t1558: f64, t18393: f64, t23160: f64, t23262: f64, t2730: f64, t2745: f64, t36833: f64, t40507: f64, t40518: f64, t40535: f64, t40868: f64, t4343: f64, t4362: f64, t4366: f64, t50532: f64, t50582: f64, t50605: f64, t5984: f64, t5988: f64, t6035: f64, t61749: f64, t61756: f64, t61774: f64, t61776: f64, t61797: f64, t61817: f64, t76474: f64, t775: f64, t800: f64, t23167: f64, t243: f64, t10726: f64, t2661: f64, t2723: f64, t18408: f64, t23334: f64, t61625: f64, t10850: f64, t221: f64, t23172: f64, t2485: f64, t23281: f64, t2652: f64, t23148: f64, t2477: f64, t40607: f64, t40611: f64, t4433: f64, t50607: f64, t50608: f64, t50611: f64, t50615: f64, t50619: f64, t50634: f64, t50681: f64, t6017: f64, t61833: f64, t61839: f64, t828: f64, t851: f64, t10858: f64, t23257: f64, t23279: f64, t10703: f64, t2674: f64, t2662: f64, t61579: f64, t10698: f64, t18392: f64, t40625: f64, t40638: f64, t40639: f64, t40654: f64, t40691: f64, t40711: f64, t50446: f64, t50703: f64, t50707: f64, t5962: f64, t5966: f64, t61860: f64, t61864: f64, t61877: f64, t1559: f64, t18608: f64, t23253: f64, t40348: f64, t10777: f64, t10779: f64, t10786: f64, t18426: f64, t18435: f64, t18627: f64, t18632: f64, t2747: f64, t40673: f64, t40722: f64, t4424: f64, t4450: f64, t50774: f64, t50957: f64, t61701: f64, t61888: f64, t61890: f64, t61892: f64, t61913: f64, t61916: f64, t61924: f64, t61952: f64, t61959: f64, t10905: f64, t23275: f64, t61956: f64, t40725: f64, t23301: f64, t125: f64, t23114: f64, t61715: f64, t10871: f64, t4423: f64, t14931: f64, t23331: f64, t10770: f64, t14676: f64, t18444: f64, t18469: f64, t18637: f64, t40664: f64, t40737: f64, t61791: f64, t837: f64, t4352: f64, t23285: f64, t2741: f64, t14494: f64, t23266: f64, t40759: f64, t40765: f64, t40771: f64, t50939: f64, t50941: f64, t61969: f64, t61973: f64, t61977: f64, t61981: f64, t61985: f64, t62012: f64, t62015: f64, t76242: f64, t76372: f64, t23289: f64, t124: f64, t40782: f64, t50943: f64, t50955: f64, t50978: f64, t62021: f64, t62029: f64, t62033: f64, t62045: f64, t62056: f64, t62058: f64, t62069: f64, t62072: f64, t62089: f64, t62095: f64, t62105: f64, t76421: f64, t799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t76517 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2831(t10811, t23297, t14923, t23336, t14894, t18525, t40455, t40489, t4364, t50436, t50443, t50454, t50457, t50505, t50524, t61689, t61692, t61697, t61699, t61718, t61727, t61754, t76284);
        let t76557 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2832(t10900, t14586, t14785, t14791, t1544, t1558, t18393, t23160, t23262, t2730, t2745, t36833, t40507, t40518, t40535, t40868, t4343, t4362, t4366, t50532, t50582, t50605, t5984, t5988, t6035, t61749, t61756, t61774, t61776, t61797, t61817, t76474, t775, t800);
        let (t76569, t76572, t76583, t76587, t76591) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2833(t23167, t243, t10726, t2661, t2723, t14586, t18408, t23334, t61625, t10850, t221, t23172, t2485);
        let t76595 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2834(t23281, t2652, t14785, t23148, t2477, t2745, t40607, t40611, t4433, t50607, t50608, t50611, t50615, t50619, t50634, t50681, t6017, t61833, t61839, t76572, t76583, t76587, t76591, t775, t828, t851);
        let t76633 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2835(t10858, t23257, t221, t23279, t10703, t2674, t2661, t2662, t6035, t61579, t10698, t1544, t18392, t2477, t40625, t40638, t40639, t40654, t40691, t40711, t4343, t50446, t50703, t50707, t5962, t5966, t61860, t61864, t61877, t775, t828, t851);
        let t76676 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2836(t1559, t18608, t2661, t2662, t23253, t40348, t10777, t10779, t5984, t10786, t18426, t18435, t18627, t18632, t2745, t2747, t40673, t40722, t4362, t4364, t4424, t4450, t50774, t50957, t61701, t61888, t61890, t61892, t61913, t61916, t61924, t61952, t61959, t76284);
        let (t76677, t76689, t76701, t76703, t76705) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2837(t10905, t23275, t10777, t10779, t6035, t61956, t1559, t40725, t5988, t14923, t23301, t125, t23114);
        let (t76726, t76742) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2838(t10777, t10779, t6035, t61715, t10871, t4423, t14931, t23334, t61956, t10811, t23331, t10770, t14676, t14894, t18426, t18444, t18469, t18637, t2723, t2745, t2747, t40664, t40673, t40737, t4343, t4362, t4364, t4366, t4424, t6017, t61791, t76284, t76677, t76689, t76701, t76703, t76705, t837);
        let t76776 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2839(t2661, t2662, t4352, t6017, t23285, t2741, t14494, t14785, t14791, t14894, t1559, t23266, t2730, t2745, t36833, t40759, t40765, t40771, t50939, t50941, t61969, t61973, t61977, t61981, t61985, t62012, t62015, t76242, t76372, t76474, t775, t800, t837);
        let t76800 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2840(t23289, t2741, t2661, t2662, t6035, t61625, t124, t40782, t50943, t50955, t50978, t62021, t62029, t62033, t62045, t62056, t62058, t62069, t62072, t62089, t62095, t62105, t76421, t799, t800);
    (t76517, t76557, t76569, t76595, t76633, t76676, t76726, t76742, t76776, t76800)
}
