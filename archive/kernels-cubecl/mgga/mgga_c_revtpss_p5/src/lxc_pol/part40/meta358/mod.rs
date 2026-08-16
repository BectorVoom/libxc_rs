//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta358 (260520-c91 hierarchical CSE).
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
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1232;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1233;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1234;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1235;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1236;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1237;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1238;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1239;
use chunk8::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1240;
use chunk9::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1241;
use chunk10::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta358<F: Float>(t14691: F, t2747: F, t837: F, t2646: F, t4450: F, t10779: F, t1548: F, t10777: F, t10811: F, t4447: F, t14676: F, t2749: F, t10673: F, t10676: F, t14668: F, t14675: F, t14678: F, t14682: F, t14690: F, t2745: F, t4362: F, t10815: F, t1561: F, t2741: F, t4426: F, t10845: F, t4430: F, t1558: F, t853: F, t2662: F, t2661: F, t4352: F, t4416: F, t221: F, t2485: F, t4424: F, t2484: F, t2652: F, t4435: F, t14663: F, t827: F, t828: F, t4343: F, t854: F, t236: F, t807: F, t124: F, t14468: F, t800: F, t775: F, t2477: F, t799: F, t825: F, t851: F, t4433: F, t10703: F, t2674: F, t4353: F, t9794: F, t10760: F, t10890: F, t1549: F, t1544: F, t2430: F, t2394: F, t10698: F, t4462: F, t808: F, t10886: F, t2703: F, t4458: F, t10678: F, t10682: F, t10687: F, t10692: F, t10769: F, t836: F, t2746: F, t14494: F, t14586: F, t10693: F, t10706: F, t10711: F, t10713: F, t10717: F, t10719: F, t10723: F, t10730: F, t10734: F, t10742: F, t2710: F, t2713: F, t4371: F, t10744: F, t10905: F, t4442: F, t4457: F, t240: F, t849: F, t14648: F, t4345: F, t10716: F, t4349: F, t10746: F, t10749: F, t10756: F, t10758: F, t2730: F, t2689: F, t4372: F, t4354: F, t9775: F, t855: F, t2675: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14693, t14697, t14703, t14705, t14707) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1232::<F>(t14691, t2747, t837, t2646, t4450, t10779, t1548, t10777, t10811, t4447, t14676, t2749);
        let t14711 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1233::<F>(t10673, t10676, t14668, t14675, t14678, t14682, t14690, t14693, t14697, t14703, t14705, t14707, t2745, t4362);
        let (t14712, t14715, t14716, t14722, t14723) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1234::<F>(t10815, t1561, t2741, t4426, t10845, t4430, t1558, t853, t2749, t2662, t2661, t4352, t837);
        let (t14726, t14730, t14734, t14736, t14738) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1235::<F>(t14723, t2662, t2661, t4416, t837, t221, t2485, t4424, t2484, t2652, t4435, t14663, t827, t828);
        let t14754 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1236::<F>(t4343, t854, t236, t807, t124, t14468, t800, t775, t2477, t828, t14712, t14715, t14716, t14722, t14726, t14730, t14734, t14736, t14738, t799, t825, t851);
        let (t14759, t14761, t14765, t14769) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1237::<F>(t221, t4433, t10703, t2674, t4353, t9794, t10760, t10890, t1549, t1544, t2430, t2477, t828);
        let t14784 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1238::<F>(t1544, t2394, t10698, t828, t10811, t4462, t4416, t808, t10886, t2703, t4458, t10678, t10682, t10687, t10692, t14759, t14761, t14765, t14769, t851);
        let t14811 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1239::<F>(t10769, t828, t1544, t836, t2749, t2746, t14494, t775, t14586, t10693, t10706, t10711, t10713, t10717, t10719, t10723, t10730, t10734, t10742, t2745, t4362);
        let (t14817, t14820, t14823, t14825, t14829) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1240::<F>(t2710, t2713, t4371, t4353, t808, t10744, t10905, t4442, t4457, t775, t800, t1548, t2430);
        let t14841 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1241::<F>(t240, t849, t14648, t775, t2661, t2652, t4345, t10716, t4349, t10746, t10749, t10756, t10758, t14817, t14820, t14823, t14825, t14829, t2730);
        let (t14843, t14846, t14850, t14853, t14857) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1242::<F>(t1548, t2394, t800, t2689, t4372, t4354, t9775, t14468, t828, t855, t221, t2675, t4343);
    (t14711, t14754, t14784, t14811, t14841, t14843, t14846, t14850, t14853, t14857)
}
