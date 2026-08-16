//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta654 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2186;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2187;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2188;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2189;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2190;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2191;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2192;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2193;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta654(t28056: f64, t4248: f64, t7933: f64, t9593: f64, t28196: f64, t28198: f64, t30138: f64, t7003: f64, t13426: f64, t7735: f64, t18227: f64, t27137: f64, t108076: f64, t108078: f64, t108080: f64, t108083: f64, t108085: f64, t108087: f64, t108089: f64, t18235: f64, t18242: f64, t25805: f64, t27145: f64, t28025: f64, t28053: f64, t5921: f64, t6985: f64, t30123: f64, t95088: f64, t670: f64, t7724: f64, t1353: f64, t6922: f64, t25082: f64, t8717: f64, t30088: f64, t689: f64, t25904: f64, t25899: f64, t30105: f64, t94395: f64, t94649: f64, t30071: f64, t7308: f64, t94378: f64, t94388: f64, t94392: f64, t97682: f64, t97687: f64, t97690: f64, t97698: f64, t97702: f64, t97707: f64, t27989: f64, t98380: f64, t6919: f64, t7242: f64, t1904: f64, t2022: f64, t22386: f64, t25924: f64, t27868: f64, t27980: f64, t28008: f64, t6895: f64, t7274: f64, t7295: f64, t7296: f64, t75188: f64, t75267: f64, t7930: f64, t94409: f64, t94580: f64, t94591: f64, t94593: f64, t97719: f64, t97734: f64, t98056: f64, t1364: f64, t30074: f64, t786: f64, t1882: f64, t543: f64, t5774: f64, t30020: f64, t686: f64, t72: f64, t25895: f64, t1398: f64, t6918: f64, t25921: f64, t25930: f64, t25931: f64, t27837: f64, t28003: f64, t30032: f64, t30096: f64, t5658: f64, t7301: f64, t75047: f64, t75051: f64, t75305: f64, t7910: f64, t7926: f64, t94602: f64, t97764: f64, t97785: f64, t98050: f64, t1955: f64, t27883: f64, t1444: f64, t27865: f64, t27869: f64, t27909: f64, t30031: f64, t30106: f64, t5728: f64, t94608: f64, t94616: f64, t94705: f64, t97792: f64, t97795: f64, t97798: f64, t97800: f64, t97804: f64, t97808: f64, t97810: f64, t97815: f64, t97933: f64, t6844: f64, t30095: f64, t1903: f64, t14224: f64, t27846: f64, t27960: f64, t30055: f64, t94635: f64, t94648: f64, t94716: f64, t97823: f64, t97825: f64, t97838: f64, t97875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108099, t108103, t108105, t108107, t108109, t108111) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2186(t28056, t4248, t7933, t9593, t28196, t28198, t30138, t7003, t13426, t7735, t18227, t27137);
        let t108114 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2187(t108076, t108078, t108080, t108083, t108085, t108087, t108089, t108099, t108103, t108105, t108107, t108109, t108111, t18235, t18242, t25805, t27145, t28025, t28053, t4248, t5921, t6985);
        let (t108117, t108120, t108129, t108133, t108135) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2188(t30123, t95088, t670, t7724, t1353, t6922, t25082, t8717, t30088, t689, t25904, t25899);
        let t108145 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2189(t30105, t689, t94395, t94649, t108133, t108135, t30071, t7308, t94378, t94388, t94392, t97682, t97687, t97690, t97698, t97702, t97707);
        let t108172 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2190(t27989, t98380, t689, t6919, t7242, t1904, t2022, t22386, t25924, t27868, t27980, t28008, t6895, t7274, t7295, t7296, t75188, t75267, t7930, t94409, t94580, t94591, t94593, t97719, t97734, t98056);
        let (t108175, t108178, t108187, t108188, t108206) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2191(t1364, t30074, t786, t1882, t543, t5774, t30020, t686, t72, t25895, t1398, t6918);
        let t108213 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2192(t108175, t108178, t108188, t108206, t25921, t25930, t25931, t27837, t27868, t27980, t28003, t30032, t30096, t543, t5658, t7295, t7301, t75047, t75051, t75305, t7910, t7926, t94602, t97764, t97785, t98050);
        let (t108225, t108233) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2193(t1955, t27883, t1444, t25924, t27865, t27869, t27909, t30031, t30106, t5728, t7295, t94608, t94616, t94705, t97792, t97795, t97798, t97800, t97804, t97808, t97810, t97815, t97933);
        let t108270 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2194(t1444, t6844, t30095, t689, t25904, t25899, t1903, t543, t5658, t14224, t1882, t25930, t25931, t27837, t27846, t27868, t27960, t30055, t30105, t7295, t7296, t7301, t94635, t94648, t94716, t97823, t97825, t97838, t97875);
    (t108114, t108117, t108120, t108129, t108145, t108172, t108187, t108213, t108225, t108233, t108270)
}
