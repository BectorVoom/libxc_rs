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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3705;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3706;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3707;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3708;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3709;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3710;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3711;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1051<F: Float>(t43865: F, t43888: F, t43890: F, t43892: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68454: F, t68456: F, t68459: F, t70158: F, t70172: F, t70186: F, t1208: F, t21332: F, t225: F, t480: F, t1235: F, t1238: F, t1250: F, t16720: F, t17254: F, t17693: F, t20945: F, t21275: F, t247: F, t371: F, t3719: F, t372: F, t44291: F, t44293: F, t482: F, t57021: F, t57026: F, t57029: F, t57520: F, t70120: F, t70129: F, t70133: F, t70140: F, t17289: F, t1803: F, t1222: F, t6652: F, t697: F, t42871: F, t6628: F, t12916: F, t17709: F, t20958: F, t1012: F, t1122: F, t17280: F, t17290: F, t17711: F, t1791: F, t20747: F, t20956: F, t3601: F, t3626: F, t3699: F, t3720: F, t44535: F, t44586: F, t5320: F, t5327: F, t57045: F, t57049: F, t57265: F, t58920: F, t59001: F, t59033: F, t60717: F, t6645: F, t676: F, t21063: F, t3678: F, t17307: F, t17225: F, t5381: F, t1261: F, t20791: F, t3172: F, t13058: F, t20786: F, t11262: F, t3711: F, t6618: F, t21110: F, t1042: F, t12784: F, t17232: F, t20792: F, t21219: F, t3647: F, t3674: F, t5268: F, t5391: F, t57063: F, t57070: F, t65433: F, t17401: F, t17620: F, t17728: F, t489: F, t5219: F, t1256: F, t21335: F, t20900: F, t3153: F, t3609: F, t69692: F, t12787: F, t12956: F, t13396: F, t16719: F, t17760: F, t17786: F, t20825: F, t21017: F, t3613: F, t3650: F, t484: F, t5302: F, t5331: F, t5333: F, t57005: F, t57075: F, t57077: F, t57094: F, t6594: F, t69763: F, t69623: F, t17202: F, t17448: F, t17558: F, t17569: F, t17669: F, t17796: F, t3610: F, t3611: F, t44170: F, t44343: F, t44698: F, t5407: F, t56254: F, t57098: F, t57100: F, t57114: F, t58983: F, t65829: F, t65947: F, t6631: F, t6635: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t70200 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3705::<F>(t43865, t43888, t43890, t43892, t56230, t56236, t68389, t68393, t68397, t68399, t68454, t68456, t68459);
        let (t70202, t70208, t70209, t70213) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3706::<F>(t70158, t70172, t70186, t70200, t1208, t21332, t225, t480, t1235, t1238, t1250, t16720, t17254, t17693, t20945, t21275, t247, t371, t3719, t372, t44291, t44293, t482, t57021, t57026, t57029, t57520, t70120, t70129, t70133, t70140);
        let (t70221, t70225, t70235) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3707::<F>(t17289, t1803, t1222, t6652, t697, t42871, t6628);
        let t70254 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3708::<F>(t12916, t17709, t20958, t1012, t1122, t1222, t1238, t17280, t17290, t17711, t1791, t20747, t20956, t3601, t3626, t3699, t3720, t44535, t44586, t5320, t5327, t57045, t57049, t57265, t58920, t59001, t59033, t60717, t70221, t70225, t70235);
        let (t70263, t70265, t70267, t70270, t70273) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3709::<F>(t1235, t371, t6645, t676, t21063, t3678, t17307, t1803, t17225, t5381, t1261, t20791, t3172);
        let t70289 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3710::<F>(t13058, t20786, t11262, t3711, t6618, t1261, t21110, t3172, t1042, t12784, t17232, t20792, t21219, t3647, t3674, t5268, t5391, t57063, t57070, t65433, t70263, t70265, t70267, t70270, t70273);
        let (t70303, t70311, t70328) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3711::<F>(t17401, t17620, t17728, t489, t5219, t1256, t21335, t20900, t3153, t3609, t69692, t1042, t12787, t12956, t13396, t16719, t17760, t17786, t20825, t21017, t3613, t3650, t3711, t3720, t484, t5302, t5331, t5333, t57005, t57075, t57077, t57094, t6594, t69763);
        let (t70343, t70361) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3712::<F>(t482, t69623, t1042, t1261, t17202, t17448, t17558, t17569, t17669, t17796, t3610, t3611, t44170, t44343, t44698, t5381, t5407, t56254, t57098, t57100, t57114, t58983, t65829, t65947, t6631, t6635);
    (t70202, t70208, t70209, t70213, t70235, t70254, t70289, t70303, t70311, t70328, t70343, t70361)
}
