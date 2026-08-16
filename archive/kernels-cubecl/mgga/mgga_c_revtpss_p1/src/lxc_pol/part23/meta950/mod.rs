//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta950 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3142;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3143;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3144;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3145;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3146;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3147;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta950<F: Float>(t1214: F, t22688: F, t21107: F, t5265: F, t1247: F, t24772: F, t3172: F, t20819: F, t5292: F, t17505: F, t20783: F, t1260: F, t24699: F, t1042: F, t1252: F, t1266: F, t17202: F, t21111: F, t21200: F, t21272: F, t21275: F, t24664: F, t3711: F, t44174: F, t5304: F, t5391: F, t69719: F, t1794: F, t5819: F, t17459: F, t23842: F, t5405: F, t24610: F, t21242: F, t5378: F, t1785: F, t21271: F, t1261: F, t24248: F, t247: F, t3634: F, t12866: F, t17261: F, t17649: F, t17693: F, t17799: F, t20932: F, t24605: F, t24647: F, t3604: F, t44510: F, t69721: F, t69773: F, t69839: F, t1264: F, t17412: F, t17763: F, t20946: F, t20947: F, t21093: F, t21153: F, t24858: F, t3617: F, t3647: F, t372: F, t5056: F, t5381: F, t5384: F, t5386: F, t6622: F, t6679: F, t6683: F, t69637: F, t71440: F, t81226: F, t21233: F, t1774: F, t4186: F, t12787: F, t17448: F, t17729: F, t20797: F, t20959: F, t20963: F, t21022: F, t21028: F, t21119: F, t21228: F, t24739: F, t3720: F, t44551: F, t44952: F, t5046: F, t57100: F, t57382: F, t6640: F, t69783: F, t70639: F, t72011: F, t17401: F, t20926: F, t4890: F, t70993: F, t17709: F, t20956: F, t24840: F, t3362: F, t3767: F, t3782: F, t4181: F, t44664: F, t5335: F, t5343: F, t5354: F, t6587: F, t69787: F, t69789: F, t69793: F, t69812: F, t71081: F, t72086: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82543, t82550, t82553, t82555, t82560, t82565) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3142::<F>(t1214, t22688, t21107, t5265, t1247, t24772, t3172, t20819, t5292, t17505, t20783, t1260, t24699);
        let t82570 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3143::<F>(t1042, t1252, t1266, t17202, t21111, t21200, t21272, t21275, t24664, t3711, t44174, t5304, t5391, t69719, t82543, t82550, t82553, t82555, t82560, t82565);
        let (t82578, t82579, t82587, t82591, t82595, t82597, t82603) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3144::<F>(t1794, t5819, t17459, t23842, t5405, t24610, t21242, t5378, t1785, t21271, t1261, t24248, t247, t3634);
        let t82608 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3145::<F>(t1266, t12866, t17261, t17649, t17693, t17799, t20932, t24605, t24647, t3604, t44510, t5405, t69721, t69773, t69839, t82579, t82587, t82591, t82595, t82597, t82603);
        let t82639 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3146::<F>(t1042, t1260, t1261, t1264, t17412, t17693, t17763, t20946, t20947, t21093, t21153, t247, t24858, t3617, t3647, t372, t5056, t5381, t5384, t5386, t5391, t6622, t6679, t6683, t69637, t71440, t81226);
        let (t82664, t82669) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3147::<F>(t21233, t5381, t1774, t4186, t12787, t17448, t17729, t20797, t20959, t20963, t21022, t21028, t21119, t21228, t24739, t3720, t44551, t44952, t5046, t57100, t57382, t6640, t69783, t70639, t72011);
        let t82696 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3148::<F>(t17401, t20926, t4890, t70993, t12787, t17709, t17729, t20956, t24840, t3362, t3720, t3767, t3782, t4181, t44664, t5335, t5343, t5354, t6587, t69787, t69789, t69793, t69812, t71081, t72086);
    (t82543, t82570, t82578, t82579, t82587, t82591, t82608, t82639, t82664, t82669, t82696)
}
