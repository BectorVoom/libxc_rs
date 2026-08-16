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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta654<F: Float>(t28056: F, t4248: F, t7933: F, t9593: F, t28196: F, t28198: F, t30138: F, t7003: F, t13426: F, t7735: F, t18227: F, t27137: F, t108076: F, t108078: F, t108080: F, t108083: F, t108085: F, t108087: F, t108089: F, t18235: F, t18242: F, t25805: F, t27145: F, t28025: F, t28053: F, t5921: F, t6985: F, t30123: F, t95088: F, t670: F, t7724: F, t1353: F, t6922: F, t25082: F, t8717: F, t30088: F, t689: F, t25904: F, t25899: F, t30105: F, t94395: F, t94649: F, t30071: F, t7308: F, t94378: F, t94388: F, t94392: F, t97682: F, t97687: F, t97690: F, t97698: F, t97702: F, t97707: F, t27989: F, t98380: F, t6919: F, t7242: F, t1904: F, t2022: F, t22386: F, t25924: F, t27868: F, t27980: F, t28008: F, t6895: F, t7274: F, t7295: F, t7296: F, t75188: F, t75267: F, t7930: F, t94409: F, t94580: F, t94591: F, t94593: F, t97719: F, t97734: F, t98056: F, t1364: F, t30074: F, t786: F, t1882: F, t543: F, t5774: F, t30020: F, t686: F, t72: F, t25895: F, t1398: F, t6918: F, t25921: F, t25930: F, t25931: F, t27837: F, t28003: F, t30032: F, t30096: F, t5658: F, t7301: F, t75047: F, t75051: F, t75305: F, t7910: F, t7926: F, t94602: F, t97764: F, t97785: F, t98050: F, t1955: F, t27883: F, t1444: F, t27865: F, t27869: F, t27909: F, t30031: F, t30106: F, t5728: F, t94608: F, t94616: F, t94705: F, t97792: F, t97795: F, t97798: F, t97800: F, t97804: F, t97808: F, t97810: F, t97815: F, t97933: F, t6844: F, t30095: F, t1903: F, t14224: F, t27846: F, t27960: F, t30055: F, t94635: F, t94648: F, t94716: F, t97823: F, t97825: F, t97838: F, t97875: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t108099, t108103, t108105, t108107, t108109, t108111) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2186::<F>(t28056, t4248, t7933, t9593, t28196, t28198, t30138, t7003, t13426, t7735, t18227, t27137);
        let t108114 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2187::<F>(t108076, t108078, t108080, t108083, t108085, t108087, t108089, t108099, t108103, t108105, t108107, t108109, t108111, t18235, t18242, t25805, t27145, t28025, t28053, t4248, t5921, t6985);
        let (t108117, t108120, t108129, t108133, t108135) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2188::<F>(t30123, t95088, t670, t7724, t1353, t6922, t25082, t8717, t30088, t689, t25904, t25899);
        let t108145 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2189::<F>(t30105, t689, t94395, t94649, t108133, t108135, t30071, t7308, t94378, t94388, t94392, t97682, t97687, t97690, t97698, t97702, t97707);
        let t108172 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2190::<F>(t27989, t98380, t689, t6919, t7242, t1904, t2022, t22386, t25924, t27868, t27980, t28008, t6895, t7274, t7295, t7296, t75188, t75267, t7930, t94409, t94580, t94591, t94593, t97719, t97734, t98056);
        let (t108175, t108178, t108187, t108188, t108206) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2191::<F>(t1364, t30074, t786, t1882, t543, t5774, t30020, t686, t72, t25895, t1398, t6918);
        let t108213 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2192::<F>(t108175, t108178, t108188, t108206, t25921, t25930, t25931, t27837, t27868, t27980, t28003, t30032, t30096, t543, t5658, t7295, t7301, t75047, t75051, t75305, t7910, t7926, t94602, t97764, t97785, t98050);
        let (t108225, t108233) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2193::<F>(t1955, t27883, t1444, t25924, t27865, t27869, t27909, t30031, t30106, t5728, t7295, t94608, t94616, t94705, t97792, t97795, t97798, t97800, t97804, t97808, t97810, t97815, t97933);
        let t108270 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2194::<F>(t1444, t6844, t30095, t689, t25904, t25899, t1903, t543, t5658, t14224, t1882, t25930, t25931, t27837, t27846, t27868, t27960, t30055, t30105, t7295, t7296, t7301, t94635, t94648, t94716, t97823, t97825, t97838, t97875);
    (t108114, t108117, t108120, t108129, t108145, t108172, t108187, t108213, t108225, t108233, t108270)
}
