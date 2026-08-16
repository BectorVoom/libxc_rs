//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta358 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1229;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1230;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1231;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1232;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1233;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1234;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1235;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1236;
use chunk8::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1237;
use chunk9::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1238;
use chunk10::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta358(t14691: f64, t2747: f64, t837: f64, t2646: f64, t4450: f64, t10779: f64, t1548: f64, t10777: f64, t10811: f64, t4447: f64, t14676: f64, t2749: f64, t10673: f64, t10676: f64, t14668: f64, t14675: f64, t14678: f64, t14682: f64, t14690: f64, t2745: f64, t4362: f64, t10815: f64, t1561: f64, t2741: f64, t4426: f64, t10845: f64, t4430: f64, t1558: f64, t853: f64, t2662: f64, t2661: f64, t4352: f64, t4416: f64, t221: f64, t2485: f64, t4424: f64, t2484: f64, t2652: f64, t4435: f64, t14663: f64, t827: f64, t828: f64, t4343: f64, t854: f64, t236: f64, t807: f64, t124: f64, t14468: f64, t800: f64, t775: f64, t2477: f64, t799: f64, t825: f64, t851: f64, t4433: f64, t10703: f64, t2674: f64, t4353: f64, t9794: f64, t10760: f64, t10890: f64, t1549: f64, t1544: f64, t2430: f64, t2394: f64, t10698: f64, t4462: f64, t808: f64, t10886: f64, t2703: f64, t4458: f64, t10678: f64, t10682: f64, t10687: f64, t10692: f64, t10769: f64, t836: f64, t2746: f64, t14494: f64, t14586: f64, t10693: f64, t10706: f64, t10711: f64, t10713: f64, t10717: f64, t10719: f64, t10723: f64, t10730: f64, t10734: f64, t10742: f64, t2710: f64, t2713: f64, t4371: f64, t10744: f64, t10905: f64, t4442: f64, t4457: f64, t240: f64, t849: f64, t14648: f64, t4345: f64, t10716: f64, t4349: f64, t10746: f64, t10749: f64, t10756: f64, t10758: f64, t2730: f64, t2689: f64, t4372: f64, t4354: f64, t9775: f64, t855: f64, t2675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14693, t14697, t14703, t14705, t14707) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1229(t14691, t2747, t837, t2646, t4450, t10779, t1548, t10777, t10811, t4447, t14676, t2749);
        let t14711 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1230(t10673, t10676, t14668, t14675, t14678, t14682, t14690, t14693, t14697, t14703, t14705, t14707, t2745, t4362);
        let (t14712, t14715, t14716, t14722, t14723) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1231(t10815, t1561, t2741, t4426, t10845, t4430, t1558, t853, t2749, t2662, t2661, t4352, t837);
        let (t14726, t14730, t14734, t14736, t14738) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1232(t14723, t2662, t2661, t4416, t837, t221, t2485, t4424, t2484, t2652, t4435, t14663, t827, t828);
        let t14754 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1233(t4343, t854, t236, t807, t124, t14468, t800, t775, t2477, t828, t14712, t14715, t14716, t14722, t14726, t14730, t14734, t14736, t14738, t799, t825, t851);
        let (t14759, t14761, t14765, t14769) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1234(t221, t4433, t10703, t2674, t4353, t9794, t10760, t10890, t1549, t1544, t2430, t2477, t828);
        let t14784 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1235(t1544, t2394, t10698, t828, t10811, t4462, t4416, t808, t10886, t2703, t4458, t10678, t10682, t10687, t10692, t14759, t14761, t14765, t14769, t851);
        let t14811 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1236(t10769, t828, t1544, t836, t2749, t2746, t14494, t775, t14586, t10693, t10706, t10711, t10713, t10717, t10719, t10723, t10730, t10734, t10742, t2745, t4362);
        let (t14817, t14820, t14823, t14825, t14829) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1237(t2710, t2713, t4371, t4353, t808, t10744, t10905, t4442, t4457, t775, t800, t1548, t2430);
        let t14841 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1238(t240, t849, t14648, t775, t2661, t2652, t4345, t10716, t4349, t10746, t10749, t10756, t10758, t14817, t14820, t14823, t14825, t14829, t2730);
        let (t14843, t14846, t14850, t14853, t14857) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1239(t1548, t2394, t800, t2689, t4372, t4354, t9775, t14468, t828, t855, t221, t2675, t4343);
    (t14711, t14754, t14784, t14811, t14841, t14843, t14846, t14850, t14853, t14857)
}
