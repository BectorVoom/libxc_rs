//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1050 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3695;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3696;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3697;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3698;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3699;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3700;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3701;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3702;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3703;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3704;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1050<F: Float>(t17217: F, t17505: F, t1032: F, t1246: F, t21333: F, t17720: F, t5391: F, t11262: F, t3610: F, t6634: F, t17569: F, t5326: F, t5390: F, t17361: F, t5293: F, t1042: F, t1252: F, t17222: F, t17547: F, t17796: F, t1797: F, t20809: F, t3363: F, t3368: F, t3617: F, t3711: F, t3714: F, t5274: F, t5287: F, t5384: F, t59371: F, t6573: F, t1261: F, t20863: F, t3172: F, t20973: F, t3647: F, t21242: F, t3636: F, t12966: F, t17261: F, t17448: F, t17609: F, t17674: F, t17679: F, t17682: F, t17684: F, t21049: F, t21200: F, t21306: F, t3626: F, t44260: F, t5331: F, t5386: F, t56835: F, t56838: F, t6425: F, t6619: F, t17306: F, t17728: F, t489: F, t12879: F, t247: F, t12772: F, t21227: F, t3625: F, t21021: F, t12855: F, t13396: F, t16714: F, t17254: F, t17461: F, t17739: F, t20795: F, t21143: F, t21203: F, t3584: F, t3603: F, t3620: F, t3720: F, t44225: F, t5402: F, t56853: F, t56867: F, t57005: F, t57275: F, t57571: F, t59411: F, t6638: F, t21007: F, t44425: F, t21222: F, t5340: F, t21101: F, t3707: F, t17426: F, t17454: F, t17456: F, t17633: F, t20797: F, t20800: F, t20956: F, t20963: F, t21020: F, t21223: F, t44252: F, t44578: F, t44664: F, t57707: F, t59375: F, t59401: F, t17608: F, t5292: F, t5265: F, t20906: F, t17416: F, t21272: F, t1260: F, t17550: F, t44264: F, t44270: F, t44273: F, t44276: F, t5268: F, t56246: F, t59241: F, t65829: F, t65947: F, t69875: F, t3568: F, t12915: F, t17344: F, t20747: F, t44693: F, t6421: F, t12910: F, t12916: F, t20857: F, t44865: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F, t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t56176: F, t56183: F, t56185: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t68363: F, t68366: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t69947, t69958, t69961, t69964, t69966, t69968) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3695::<F>(t17217, t17505, t1032, t1246, t21333, t17720, t5391, t11262, t3610, t6634, t17569, t5326, t5390);
        let t69982 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3696::<F>(t17361, t5293, t1042, t1252, t17222, t17505, t17547, t17796, t1797, t20809, t3363, t3368, t3617, t3711, t3714, t5274, t5287, t5384, t59371, t6573, t69947, t69958, t69961, t69964, t69966, t69968);
        let t70011 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3697::<F>(t1261, t20863, t3172, t20973, t3647, t21242, t3636, t12966, t17261, t17448, t17609, t17674, t17679, t17682, t17684, t21049, t21200, t21306, t3626, t44260, t5287, t5331, t5386, t5390, t56835, t56838, t6425, t6619);
        let t70050 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3698::<F>(t17306, t17728, t489, t1261, t12879, t247, t6425, t12772, t21227, t3625, t21021, t12855, t13396, t16714, t17254, t17461, t17739, t20795, t21143, t21203, t3584, t3603, t3620, t3626, t3720, t44225, t5402, t56853, t56867, t57005, t57275, t57571, t59411, t6638);
        let t70085 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3699::<F>(t21007, t3625, t44425, t12772, t21222, t5340, t21101, t3707, t1252, t12855, t17222, t17426, t17454, t17456, t17633, t1797, t20797, t20800, t20956, t20963, t21020, t21223, t3626, t3720, t44252, t44578, t44664, t5293, t57707, t59375, t59401);
        let t70119 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3700::<F>(t17608, t5292, t17547, t5265, t1261, t20906, t3172, t17416, t5391, t21272, t3636, t1042, t1252, t1260, t17550, t44264, t44270, t44273, t44276, t5268, t5384, t5386, t56246, t59241, t65829, t65947, t69875);
        let (t70120, t70129, t70133, t70140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3701::<F>(t3568, t6573, t12915, t17344, t20747, t247, t1261, t44693, t6421, t12910, t12916, t20857);
        let t70158 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3702::<F>(t44865, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t70172 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3703::<F>(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t70186 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3704::<F>(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
    (t69982, t70011, t70050, t70085, t70119, t70120, t70129, t70133, t70140, t70158, t70172, t70186)
}
