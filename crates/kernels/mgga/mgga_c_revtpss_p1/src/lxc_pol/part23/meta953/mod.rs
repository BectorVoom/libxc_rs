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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta953<F: Float>(t1208: F, t24697: F, t225: F, t480: F, t17438: F, t20846: F, t5326: F, t6594: F, t1238: F, t12787: F, t17183: F, t17736: F, t17934: F, t21013: F, t21046: F, t24729: F, t3626: F, t3720: F, t5230: F, t5297: F, t5335: F, t5340: F, t5343: F, t6421: F, t70064: F, t70076: F, t70311: F, t70530: F, t71029: F, t3617: F, t372: F, t6628: F, t20973: F, t5391: F, t5381: F, t12916: F, t24735: F, t5331: F, t12809: F, t17351: F, t17661: F, t21222: F, t21246: F, t21267: F, t21275: F, t3611: F, t44264: F, t44510: F, t5047: F, t70091: F, t70102: F, t70959: F, t82321: F, t12855: F, t24835: F, t17729: F, t20317: F, t20802: F, t20806: F, t20952: F, t20978: F, t21049: F, t21306: F, t24734: F, t59162: F, t70112: F, t70114: F, t70129: F, t70133: F, t82481: F, t44307: F, t68255: F, t68257: F, t68262: F, t68277: F, t81156: F, t81158: F, t81162: F, t81167: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F, t43888: F, t56176: F, t56236: F, t56447: F, t56462: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F, t459: F, t1230: F, t17396: F, t20265: F, t20797: F, t20959: F, t20963: F, t21300: F, t24680: F, t4181: F, t484: F, t5348: F, t5354: F, t57005: F, t57710: F, t6425: F, t70140: F, t70800: F, t71036: F, t71039: F, t71081: F, t1803: F, t20923: F, t21334: F, t44291: F, t5261: F, t70225: F, t70250: F, t70263: F, t70265: F, t70270: F, t70273: F, t70275: F, t70647: F, t1012: F, t1222: F, t1225: F, t17649: F, t17654: F, t20767: F, t20938: F, t21111: F, t21119: F, t21210: F, t5373: F, t57094: F, t70278: F, t70281: F, t70300: F, t70306: F, t70990: F, t71440: F, t76397: F, t83033: F, t1038: F, t1241: F, t1244: F, t24679: F, t1252: F, t17693: F, t17799: F, t1797: F, t21028: F, t21102: F, t5287: F, t57118: F, t69958: F, t70082: F, t70088: F, t70369: F, t70373: F, t70376: F, t83034: F, t21213: F, t5357: F, t17401: F, t21166: F, t21259: F, t57126: F, t70378: F, t70382: F, t70394: F, t70403: F, t70405: F, t70411: F, t70427: F, t70432: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t83107, t83108, t83117) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3163::<F>(t1208, t24697, t225, t480, t17438, t20846, t5326, t6594, t1238, t12787, t17183, t17736, t17934, t21013, t21046, t24729, t3626, t3720, t5230, t5297, t5335, t5340, t5343, t6421, t70064, t70076, t70311, t70530, t71029);
        let (t83125, t83145) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3164::<F>(t3617, t372, t6628, t20973, t5391, t5381, t12916, t24735, t5331, t12809, t17351, t17661, t21222, t21246, t21267, t21275, t3611, t3720, t44264, t44510, t5047, t70091, t70102, t70959, t82321);
        let t83170 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3165::<F>(t12855, t12916, t24835, t17729, t20317, t20802, t20806, t20952, t20978, t21049, t21306, t24734, t3626, t3720, t5331, t59162, t70112, t70114, t70129, t70133, t70311, t82481);
        let t83211 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3166::<F>(t44307, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t83230 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3167::<F>(t43888, t56176, t56236, t56447, t56462, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
        let (t83232, t83240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3168::<F>(t459, t83211, t83230, t1230, t17396, t17736, t20265, t20797, t20959, t20963, t21300, t225, t24680, t3626, t4181, t480, t484, t5230, t5348, t5354, t57005, t57710, t6425, t70140, t70800, t71036, t71039, t71081);
        let t83259 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3169::<F>(t1803, t20923, t21334, t44291, t484, t5261, t6594, t70225, t70250, t70263, t70265, t70270, t70273, t70275, t70647);
        let t83281 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3170::<F>(t1012, t1222, t1225, t17649, t17654, t20767, t20938, t21111, t21119, t21210, t5373, t5381, t57094, t70278, t70281, t70300, t70306, t70990, t71440, t76397, t83033);
        let t83307 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3171::<F>(t1038, t1241, t1244, t24679, t1252, t17351, t17649, t17693, t17799, t1797, t21028, t21102, t5287, t57118, t69958, t70082, t70088, t70369, t70373, t70376, t83033, t83034);
        let t83322 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3172::<F>(t21213, t5357, t17401, t21166, t21259, t57126, t70378, t70382, t70394, t70403, t70405, t70411, t70427, t70432);
    (t83107, t83108, t83117, t83125, t83145, t83170, t83232, t83240, t83259, t83281, t83307, t83322)
}
