//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1589;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1590;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1591;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1592;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1593;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta438(t5854: f64, t607: f64, t10355: f64, t5819: f64, t606: f64, t4186: f64, t4201: f64, t2275: f64, t5825: f64, t18281: f64, t48: f64, t10368: f64, t4210: f64, t2282: f64, t60: f64, t10379: f64, t1480: f64, t4211: f64, t4214: f64, t44: f64, t56: f64, t5835: f64, t5838: f64, t5843: f64, t614: f64, t620: f64, t38: f64, t10389: f64, t2299: f64, t10398: f64, t2306: f64, t4227: f64, t4232: f64, t633: f64, t637: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t4188: f64, t4191: f64, t4196: f64, t4218: f64, t4238: f64, t5855: f64, t5869: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t21720: f64, t10301: f64, t10309: f64, t13269: f64, t13272: f64, t1497: f64, t21661: f64, t21663: f64, t21674: f64, t21677: f64, t21682: f64, t2242: f64, t2247: f64, t4173: f64, t4178: f64, t4241: f64, t5816: f64, t5872: f64, t603: f64, t644: f64, t91: f64, t5: f64, t117: f64, t5892: f64, t625: f64, t10208: f64, t5891: f64, t665: f64, t4263: f64, t4287: f64, t5916: f64, t2339: f64, t5915: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21727, t21733, t21736, t21742, t21745, t21754) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1589(t5854, t607, t10355, t5819, t606, t4186, t4201, t2275, t5825, t18281, t48, t10368);
        let t21768 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1590(t21754, t606, t4186, t4210, t2282, t5825, t18281, t60, t10379, t1480, t21733, t21736, t21742, t21745, t4211, t4214, t44, t56, t5835, t5838, t5843, t614, t620);
        let (t21769, t21804) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1591(t21768, t38, t10389, t5819, t2299, t5825, t10398, t2306, t18281, t4186, t4227, t4232, t606, t633, t637);
        let t21808 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1592(t21804, t77, t1471, t1487, t1494, t21727, t21769, t4188, t4191, t4196, t4218, t4238, t5855, t5869, t608, t628, t641, t71, t85);
        let (t21809, t21812) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1593(t21720, t21808, t10301, t10309, t13269, t13272, t1497, t21661, t21663, t21674, t21677, t21682, t2242, t2247, t4173, t4178, t4241, t5816, t5872, t603, t644, t91);
        let (t21813, t21814, t21818, t21821, t21824, t21827, t21829) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1594(t5, t21812, t117, t5892, t625, t10208, t5891, t665, t4263, t4287, t5916, t2339, t5915);
    (t21804, t21809, t21813, t21814, t21818, t21821, t21824, t21827, t21829)
}
