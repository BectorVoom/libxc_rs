//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta406 (260520-c91 hierarchical CSE).
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
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1412;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1413;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1414;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1415;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1416;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1417;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1418;
use chunk7::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1419;
use chunk8::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1420;
use chunk9::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1421;
use chunk10::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1422;
use chunk11::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta406(t5854: f64, t607: f64, t10355: f64, t5819: f64, t606: f64, t4186: f64, t4201: f64, t2275: f64, t5825: f64, t18281: f64, t48: f64, t10368: f64, t4210: f64, t2282: f64, t60: f64, t10379: f64, t1480: f64, t4211: f64, t4214: f64, t44: f64, t56: f64, t5835: f64, t5838: f64, t5843: f64, t614: f64, t620: f64, t38: f64, t10389: f64, t2299: f64, t10398: f64, t2306: f64, t4227: f64, t4232: f64, t633: f64, t637: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t4188: f64, t4191: f64, t4196: f64, t4218: f64, t4238: f64, t5855: f64, t5869: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t21720: f64, t10301: f64, t10309: f64, t13269: f64, t13272: f64, t1497: f64, t21661: f64, t21663: f64, t21674: f64, t21677: f64, t21682: f64, t2242: f64, t2247: f64, t4173: f64, t4178: f64, t4241: f64, t5816: f64, t5872: f64, t603: f64, t644: f64, t91: f64, t5: f64, t117: f64, t5892: f64, t625: f64, t10208: f64, t5891: f64, t665: f64, t4263: f64, t4287: f64, t5916: f64, t2339: f64, t5915: f64, t10227: f64, t5895: f64, t658: f64, t1504: f64, t2: f64, t580: f64, t2349: f64, t5823: f64, t9342: f64, t100: f64, t10241: f64, t5907: f64, t661: f64, t1509: f64, t2357: f64, t5911: f64, t108: f64, t105: f64, t13475: f64, t13496: f64, t1507: f64, t4280: f64, t4284: f64, t5896: f64, t5899: f64, t5902: f64, t656: f64, t662: f64, t97: f64, t655: f64, t10201: f64, t10202: f64, t13448: f64, t13451: f64, t13453: f64, t69: f64, t114: f64, t30: f64, t508: f64, t1518: f64, t5517: f64, t13584: f64, t9375: f64, t6785: f64, t9335: f64, t3833: f64, t5824: f64, t18280: f64, t2255: f64, t513: f64, t5549: f64, t605: f64, zeta_threshold: f64, t33: f64, t6792: f64, t9350: f64, t3841: f64, t6416: f64, t1113: f64, t20256: f64, t516: f64, t5557: f64, t162: f64, t187: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21727, t21733, t21736, t21742, t21745, t21754) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1412(t5854, t607, t10355, t5819, t606, t4186, t4201, t2275, t5825, t18281, t48, t10368);
        let t21768 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1413(t21754, t606, t4186, t4210, t2282, t5825, t18281, t60, t10379, t1480, t21733, t21736, t21742, t21745, t4211, t4214, t44, t56, t5835, t5838, t5843, t614, t620);
        let (t21769, t21804) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1414(t21768, t38, t10389, t5819, t2299, t5825, t10398, t2306, t18281, t4186, t4227, t4232, t606, t633, t637);
        let t21808 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1415(t21804, t77, t1471, t1487, t1494, t21727, t21769, t4188, t4191, t4196, t4218, t4238, t5855, t5869, t608, t628, t641, t71, t85);
        let t21812 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1416(t21720, t21808, t10301, t10309, t13269, t13272, t1497, t21661, t21663, t21674, t21677, t21682, t2242, t2247, t4173, t4178, t4241, t5816, t5872, t603, t644, t91);
        let (t21813, t21814, t21818, t21821, t21824, t21827, t21829) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1417(t5, t21812, t117, t5892, t625, t10208, t5891, t665, t4263, t4287, t5916, t2339, t5915);
        let (t21830, t21836, t21840, t21846, t21850, t21851) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1418(t21829, t665, t10227, t5895, t658, t1504, t2, t580, t2349, t5823, t9342, t100);
        let (t21864, t21872, t21876) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1419(t10241, t5907, t661, t1509, t2, t580, t2357, t5911, t21850, t108, t105, t13475, t13496, t1507, t21836, t21840, t21846, t21851, t4280, t4284, t5896, t5899, t5902, t656, t662, t97);
        let t21880 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1420(t21876, t655, t10201, t10202, t13448, t13451, t13453, t21818, t21821, t21824, t21827, t21830, t69);
        let t21881 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1421(t114, t21880);
        let (t21882, t21891, t21901, t21905, t21917) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1422(t30, t21881, t508, t1518, t5517, t13584, t9375, t6785, t9335, t3833, t5824, t18280, t2255, t513, t5549, t605, zeta_threshold);
        let (t21931, t21933) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1423(t33, t6792, t9350, t3841, t6416, t1113, t20256, t2255, t516, t5557, t162, t21917, t187, zeta_threshold);
    (t21813, t21814, t21864, t21872, t21876, t21881, t21882, t21891, t21901, t21905, t21931, t21933)
}
