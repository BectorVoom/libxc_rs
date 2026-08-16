//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1051 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3705;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3706;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3707;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3708;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3709;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3710;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3711;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1051(t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64, t70158: f64, t70172: f64, t70186: f64, t1208: f64, t21332: f64, t225: f64, t480: f64, t1235: f64, t1238: f64, t1250: f64, t16720: f64, t17254: f64, t17693: f64, t20945: f64, t21275: f64, t247: f64, t371: f64, t3719: f64, t372: f64, t44291: f64, t44293: f64, t482: f64, t57021: f64, t57026: f64, t57029: f64, t57520: f64, t70120: f64, t70129: f64, t70133: f64, t70140: f64, t17289: f64, t1803: f64, t1222: f64, t6652: f64, t697: f64, t42871: f64, t6628: f64, t12916: f64, t17709: f64, t20958: f64, t1012: f64, t1122: f64, t17280: f64, t17290: f64, t17711: f64, t1791: f64, t20747: f64, t20956: f64, t3601: f64, t3626: f64, t3699: f64, t3720: f64, t44535: f64, t44586: f64, t5320: f64, t5327: f64, t57045: f64, t57049: f64, t57265: f64, t58920: f64, t59001: f64, t59033: f64, t60717: f64, t6645: f64, t676: f64, t21063: f64, t3678: f64, t17307: f64, t17225: f64, t5381: f64, t1261: f64, t20791: f64, t3172: f64, t13058: f64, t20786: f64, t11262: f64, t3711: f64, t6618: f64, t21110: f64, t1042: f64, t12784: f64, t17232: f64, t20792: f64, t21219: f64, t3647: f64, t3674: f64, t5268: f64, t5391: f64, t57063: f64, t57070: f64, t65433: f64, t17401: f64, t17620: f64, t17728: f64, t489: f64, t5219: f64, t1256: f64, t21335: f64, t20900: f64, t3153: f64, t3609: f64, t69692: f64, t12787: f64, t12956: f64, t13396: f64, t16719: f64, t17760: f64, t17786: f64, t20825: f64, t21017: f64, t3613: f64, t3650: f64, t484: f64, t5302: f64, t5331: f64, t5333: f64, t57005: f64, t57075: f64, t57077: f64, t57094: f64, t6594: f64, t69763: f64, t69623: f64, t17202: f64, t17448: f64, t17558: f64, t17569: f64, t17669: f64, t17796: f64, t3610: f64, t3611: f64, t44170: f64, t44343: f64, t44698: f64, t5407: f64, t56254: f64, t57098: f64, t57100: f64, t57114: f64, t58983: f64, t65829: f64, t65947: f64, t6631: f64, t6635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t70200 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3705(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t70202, t70208, t70209, t70213) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3706(t70158, t70172, t70186, t70200, t1208, t21332, t225, t480, t1235, t1238, t1250, t16720, t17254, t17693, t20945, t21275, t247, t371, t3719, t372, t44291, t44293, t482, t57021, t57026, t57029, t57520, t70120, t70129, t70133, t70140);
        let (t70221, t70225, t70235) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3707(t17289, t1803, t1222, t6652, t697, t42871, t6628);
        let t70254 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3708(t12916, t17709, t20958, t1012, t1122, t1222, t1238, t17280, t17290, t17711, t1791, t20747, t20956, t3601, t3626, t3699, t3720, t44535, t44586, t5320, t5327, t57045, t57049, t57265, t58920, t59001, t59033, t60717, t70221, t70225, t70235);
        let (t70263, t70265, t70267, t70270, t70273) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3709(t1235, t371, t6645, t676, t21063, t3678, t17307, t1803, t17225, t5381, t1261, t20791, t3172);
        let t70289 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3710(t13058, t20786, t11262, t3711, t6618, t1261, t21110, t3172, t1042, t12784, t17232, t20792, t21219, t3647, t3674, t5268, t5391, t57063, t57070, t65433, t70263, t70265, t70267, t70270, t70273);
        let (t70303, t70311, t70328) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3711(t17401, t17620, t17728, t489, t5219, t1256, t21335, t20900, t3153, t3609, t69692, t1042, t12787, t12956, t13396, t16719, t17760, t17786, t20825, t21017, t3613, t3650, t3711, t3720, t484, t5302, t5331, t5333, t57005, t57075, t57077, t57094, t6594, t69763);
        let (t70343, t70361) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3712(t482, t69623, t1042, t1261, t17202, t17448, t17558, t17569, t17669, t17796, t3610, t3611, t44170, t44343, t44698, t5381, t5407, t56254, t57098, t57100, t57114, t58983, t65829, t65947, t6631, t6635);
    (t70202, t70208, t70209, t70213, t70235, t70254, t70289, t70303, t70311, t70328, t70343, t70361)
}
