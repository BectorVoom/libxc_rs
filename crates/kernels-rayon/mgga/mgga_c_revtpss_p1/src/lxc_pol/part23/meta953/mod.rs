//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta953 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3163;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3164;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3165;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3166;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3167;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3168;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3169;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3170;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3171;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta953(t1208: f64, t24697: f64, t225: f64, t480: f64, t17438: f64, t20846: f64, t5326: f64, t6594: f64, t1238: f64, t12787: f64, t17183: f64, t17736: f64, t17934: f64, t21013: f64, t21046: f64, t24729: f64, t3626: f64, t3720: f64, t5230: f64, t5297: f64, t5335: f64, t5340: f64, t5343: f64, t6421: f64, t70064: f64, t70076: f64, t70311: f64, t70530: f64, t71029: f64, t3617: f64, t372: f64, t6628: f64, t20973: f64, t5391: f64, t5381: f64, t12916: f64, t24735: f64, t5331: f64, t12809: f64, t17351: f64, t17661: f64, t21222: f64, t21246: f64, t21267: f64, t21275: f64, t3611: f64, t44264: f64, t44510: f64, t5047: f64, t70091: f64, t70102: f64, t70959: f64, t82321: f64, t12855: f64, t24835: f64, t17729: f64, t20317: f64, t20802: f64, t20806: f64, t20952: f64, t20978: f64, t21049: f64, t21306: f64, t24734: f64, t59162: f64, t70112: f64, t70114: f64, t70129: f64, t70133: f64, t82481: f64, t44307: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64, t43888: f64, t56176: f64, t56236: f64, t56447: f64, t56462: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64, t459: f64, t1230: f64, t17396: f64, t20265: f64, t20797: f64, t20959: f64, t20963: f64, t21300: f64, t24680: f64, t4181: f64, t484: f64, t5348: f64, t5354: f64, t57005: f64, t57710: f64, t6425: f64, t70140: f64, t70800: f64, t71036: f64, t71039: f64, t71081: f64, t1803: f64, t20923: f64, t21334: f64, t44291: f64, t5261: f64, t70225: f64, t70250: f64, t70263: f64, t70265: f64, t70270: f64, t70273: f64, t70275: f64, t70647: f64, t1012: f64, t1222: f64, t1225: f64, t17649: f64, t17654: f64, t20767: f64, t20938: f64, t21111: f64, t21119: f64, t21210: f64, t5373: f64, t57094: f64, t70278: f64, t70281: f64, t70300: f64, t70306: f64, t70990: f64, t71440: f64, t76397: f64, t83033: f64, t1038: f64, t1241: f64, t1244: f64, t24679: f64, t1252: f64, t17693: f64, t17799: f64, t1797: f64, t21028: f64, t21102: f64, t5287: f64, t57118: f64, t69958: f64, t70082: f64, t70088: f64, t70369: f64, t70373: f64, t70376: f64, t83034: f64, t21213: f64, t5357: f64, t17401: f64, t21166: f64, t21259: f64, t57126: f64, t70378: f64, t70382: f64, t70394: f64, t70403: f64, t70405: f64, t70411: f64, t70427: f64, t70432: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83107, t83108, t83117) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3163(t1208, t24697, t225, t480, t17438, t20846, t5326, t6594, t1238, t12787, t17183, t17736, t17934, t21013, t21046, t24729, t3626, t3720, t5230, t5297, t5335, t5340, t5343, t6421, t70064, t70076, t70311, t70530, t71029);
        let (t83125, t83145) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3164(t3617, t372, t6628, t20973, t5391, t5381, t12916, t24735, t5331, t12809, t17351, t17661, t21222, t21246, t21267, t21275, t3611, t3720, t44264, t44510, t5047, t70091, t70102, t70959, t82321);
        let t83170 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3165(t12855, t12916, t24835, t17729, t20317, t20802, t20806, t20952, t20978, t21049, t21306, t24734, t3626, t3720, t5331, t59162, t70112, t70114, t70129, t70133, t70311, t82481);
        let t83211 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3166(t44307, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t83230 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3167(t43888, t56176, t56236, t56447, t56462, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
        let (t83232, t83240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3168(t459, t83211, t83230, t1230, t17396, t17736, t20265, t20797, t20959, t20963, t21300, t225, t24680, t3626, t4181, t480, t484, t5230, t5348, t5354, t57005, t57710, t6425, t70140, t70800, t71036, t71039, t71081);
        let t83259 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3169(t1803, t20923, t21334, t44291, t484, t5261, t6594, t70225, t70250, t70263, t70265, t70270, t70273, t70275, t70647);
        let t83281 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3170(t1012, t1222, t1225, t17649, t17654, t20767, t20938, t21111, t21119, t21210, t5373, t5381, t57094, t70278, t70281, t70300, t70306, t70990, t71440, t76397, t83033);
        let t83307 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3171(t1038, t1241, t1244, t24679, t1252, t17351, t17649, t17693, t17799, t1797, t21028, t21102, t5287, t57118, t69958, t70082, t70088, t70369, t70373, t70376, t83033, t83034);
        let t83322 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3172(t21213, t5357, t17401, t21166, t21259, t57126, t70378, t70382, t70394, t70403, t70405, t70411, t70427, t70432);
    (t83107, t83108, t83117, t83125, t83145, t83170, t83232, t83240, t83259, t83281, t83307, t83322)
}
