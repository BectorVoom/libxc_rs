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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1050(t17217: f64, t17505: f64, t1032: f64, t1246: f64, t21333: f64, t17720: f64, t5391: f64, t11262: f64, t3610: f64, t6634: f64, t17569: f64, t5326: f64, t5390: f64, t17361: f64, t5293: f64, t1042: f64, t1252: f64, t17222: f64, t17547: f64, t17796: f64, t1797: f64, t20809: f64, t3363: f64, t3368: f64, t3617: f64, t3711: f64, t3714: f64, t5274: f64, t5287: f64, t5384: f64, t59371: f64, t6573: f64, t1261: f64, t20863: f64, t3172: f64, t20973: f64, t3647: f64, t21242: f64, t3636: f64, t12966: f64, t17261: f64, t17448: f64, t17609: f64, t17674: f64, t17679: f64, t17682: f64, t17684: f64, t21049: f64, t21200: f64, t21306: f64, t3626: f64, t44260: f64, t5331: f64, t5386: f64, t56835: f64, t56838: f64, t6425: f64, t6619: f64, t17306: f64, t17728: f64, t489: f64, t12879: f64, t247: f64, t12772: f64, t21227: f64, t3625: f64, t21021: f64, t12855: f64, t13396: f64, t16714: f64, t17254: f64, t17461: f64, t17739: f64, t20795: f64, t21143: f64, t21203: f64, t3584: f64, t3603: f64, t3620: f64, t3720: f64, t44225: f64, t5402: f64, t56853: f64, t56867: f64, t57005: f64, t57275: f64, t57571: f64, t59411: f64, t6638: f64, t21007: f64, t44425: f64, t21222: f64, t5340: f64, t21101: f64, t3707: f64, t17426: f64, t17454: f64, t17456: f64, t17633: f64, t20797: f64, t20800: f64, t20956: f64, t20963: f64, t21020: f64, t21223: f64, t44252: f64, t44578: f64, t44664: f64, t57707: f64, t59375: f64, t59401: f64, t17608: f64, t5292: f64, t5265: f64, t20906: f64, t17416: f64, t21272: f64, t1260: f64, t17550: f64, t44264: f64, t44270: f64, t44273: f64, t44276: f64, t5268: f64, t56246: f64, t59241: f64, t65829: f64, t65947: f64, t69875: f64, t3568: f64, t12915: f64, t17344: f64, t20747: f64, t44693: f64, t6421: f64, t12910: f64, t12916: f64, t20857: f64, t44865: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64, t68297: f64, t68301: f64, t68305: f64, t68310: f64, t68332: f64, t68334: f64, t68336: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64, t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69947, t69958, t69961, t69964, t69966, t69968) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3695(t17217, t17505, t1032, t1246, t21333, t17720, t5391, t11262, t3610, t6634, t17569, t5326, t5390);
        let t69982 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3696(t17361, t5293, t1042, t1252, t17222, t17505, t17547, t17796, t1797, t20809, t3363, t3368, t3617, t3711, t3714, t5274, t5287, t5384, t59371, t6573, t69947, t69958, t69961, t69964, t69966, t69968);
        let t70011 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3697(t1261, t20863, t3172, t20973, t3647, t21242, t3636, t12966, t17261, t17448, t17609, t17674, t17679, t17682, t17684, t21049, t21200, t21306, t3626, t44260, t5287, t5331, t5386, t5390, t56835, t56838, t6425, t6619);
        let t70050 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3698(t17306, t17728, t489, t1261, t12879, t247, t6425, t12772, t21227, t3625, t21021, t12855, t13396, t16714, t17254, t17461, t17739, t20795, t21143, t21203, t3584, t3603, t3620, t3626, t3720, t44225, t5402, t56853, t56867, t57005, t57275, t57571, t59411, t6638);
        let t70085 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3699(t21007, t3625, t44425, t12772, t21222, t5340, t21101, t3707, t1252, t12855, t17222, t17426, t17454, t17456, t17633, t1797, t20797, t20800, t20956, t20963, t21020, t21223, t3626, t3720, t44252, t44578, t44664, t5293, t57707, t59375, t59401);
        let t70119 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3700(t17608, t5292, t17547, t5265, t1261, t20906, t3172, t17416, t5391, t21272, t3636, t1042, t1252, t1260, t17550, t44264, t44270, t44273, t44276, t5268, t5384, t5386, t56246, t59241, t65829, t65947, t69875);
        let (t70120, t70129, t70133, t70140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3701(t3568, t6573, t12915, t17344, t20747, t247, t1261, t44693, t6421, t12910, t12916, t20857);
        let t70158 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3702(t44865, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282, t68287, t68292);
        let t70172 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3703(t68297, t68301, t68305, t68310, t68332, t68334, t68336, t68342, t68347, t68350, t68353, t68357, t68360);
        let t70186 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3704(t56176, t56183, t56185, t56187, t56189, t56209, t56212, t56214, t56216, t56228, t68363, t68366);
    (t69982, t70011, t70050, t70085, t70119, t70120, t70129, t70133, t70140, t70158, t70172, t70186)
}
