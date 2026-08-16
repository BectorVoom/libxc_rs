//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta950 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3142;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3143;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3144;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3145;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3146;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3147;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta950(t1214: f64, t22688: f64, t21107: f64, t5265: f64, t1247: f64, t24772: f64, t3172: f64, t20819: f64, t5292: f64, t17505: f64, t20783: f64, t1260: f64, t24699: f64, t1042: f64, t1252: f64, t1266: f64, t17202: f64, t21111: f64, t21200: f64, t21272: f64, t21275: f64, t24664: f64, t3711: f64, t44174: f64, t5304: f64, t5391: f64, t69719: f64, t1794: f64, t5819: f64, t17459: f64, t23842: f64, t5405: f64, t24610: f64, t21242: f64, t5378: f64, t1785: f64, t21271: f64, t1261: f64, t24248: f64, t247: f64, t3634: f64, t12866: f64, t17261: f64, t17649: f64, t17693: f64, t17799: f64, t20932: f64, t24605: f64, t24647: f64, t3604: f64, t44510: f64, t69721: f64, t69773: f64, t69839: f64, t1264: f64, t17412: f64, t17763: f64, t20946: f64, t20947: f64, t21093: f64, t21153: f64, t24858: f64, t3617: f64, t3647: f64, t372: f64, t5056: f64, t5381: f64, t5384: f64, t5386: f64, t6622: f64, t6679: f64, t6683: f64, t69637: f64, t71440: f64, t81226: f64, t21233: f64, t1774: f64, t4186: f64, t12787: f64, t17448: f64, t17729: f64, t20797: f64, t20959: f64, t20963: f64, t21022: f64, t21028: f64, t21119: f64, t21228: f64, t24739: f64, t3720: f64, t44551: f64, t44952: f64, t5046: f64, t57100: f64, t57382: f64, t6640: f64, t69783: f64, t70639: f64, t72011: f64, t17401: f64, t20926: f64, t4890: f64, t70993: f64, t17709: f64, t20956: f64, t24840: f64, t3362: f64, t3767: f64, t3782: f64, t4181: f64, t44664: f64, t5335: f64, t5343: f64, t5354: f64, t6587: f64, t69787: f64, t69789: f64, t69793: f64, t69812: f64, t71081: f64, t72086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82543, t82550, t82553, t82555, t82560, t82565) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3142(t1214, t22688, t21107, t5265, t1247, t24772, t3172, t20819, t5292, t17505, t20783, t1260, t24699);
        let t82570 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3143(t1042, t1252, t1266, t17202, t21111, t21200, t21272, t21275, t24664, t3711, t44174, t5304, t5391, t69719, t82543, t82550, t82553, t82555, t82560, t82565);
        let (t82578, t82579, t82587, t82591, t82595, t82597, t82603) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3144(t1794, t5819, t17459, t23842, t5405, t24610, t21242, t5378, t1785, t21271, t1261, t24248, t247, t3634);
        let t82608 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3145(t1266, t12866, t17261, t17649, t17693, t17799, t20932, t24605, t24647, t3604, t44510, t5405, t69721, t69773, t69839, t82579, t82587, t82591, t82595, t82597, t82603);
        let t82639 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3146(t1042, t1260, t1261, t1264, t17412, t17693, t17763, t20946, t20947, t21093, t21153, t247, t24858, t3617, t3647, t372, t5056, t5381, t5384, t5386, t5391, t6622, t6679, t6683, t69637, t71440, t81226);
        let (t82664, t82669) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3147(t21233, t5381, t1774, t4186, t12787, t17448, t17729, t20797, t20959, t20963, t21022, t21028, t21119, t21228, t24739, t3720, t44551, t44952, t5046, t57100, t57382, t6640, t69783, t70639, t72011);
        let t82696 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3148(t17401, t20926, t4890, t70993, t12787, t17709, t17729, t20956, t24840, t3362, t3720, t3767, t3782, t4181, t44664, t5335, t5343, t5354, t6587, t69787, t69789, t69793, t69812, t71081, t72086);
    (t82543, t82570, t82578, t82579, t82587, t82591, t82608, t82639, t82664, t82669, t82696)
}
