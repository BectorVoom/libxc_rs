//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta441 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1675;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1676;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1677;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1678;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1679;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1680;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1681;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1682;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1683;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1684;
use chunk10::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1685;
use chunk11::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta441<F: Float>(t12657: F, t3754: F, t12722: F, t3555: F, t12640: F, t3552: F, t3766: F, t5462: F, t5477: F, t12699: F, t12709: F, t12714: F, t12719: F, t12723: F, t12727: F, t12748: F, t12753: F, t12757: F, t17955: F, t3756: F, t3770: F, t3778: F, t44639: F, t5478: F, t5480: F, t12690: F, t1284: F, t3601: F, t3727: F, t1209: F, t17948: F, t12050: F, t471: F, t1214: F, t3588: F, t12744: F, t1280: F, t1288: F, t13121: F, t13148: F, t13149: F, t13153: F, t17888: F, t17949: F, t3666: F, t3670: F, t3767: F, t3769: F, t3774: F, t44501: F, t44585: F, t44944: F, t45391: F, t45584: F, t3781: F, t1204: F, t13147: F, t13141: F, t3596: F, t42859: F, t460: F, t3603: F, t43351: F, t17703: F, t1248: F, t12629: F, t12646: F, t12712: F, t12717: F, t12734: F, t12741: F, t12751: F, t1287: F, t12975: F, t12987: F, t13144: F, t13150: F, t3302: F, t3746: F, t3759: F, t3760: F, t3782: F, t3783: F, t3784: F, t45609: F, t45648: F, t1243: F, t1234: F, t1269: F, t12732: F, t1285: F, t12966: F, t13127: F, t13129: F, t13156: F, t3787: F, t44778: F, t44843: F, t44845: F, t44878: F, t45329: F, t45406: F, t487: F, t489: F, t5463: F, t5465: F, t13126: F, t3566: F, t12621: F, t12706: F, t12769: F, t1281: F, t13130: F, t13133: F, t13161: F, t17864: F, t3568: F, t3763: F, t44552: F, t44832: F, t45385: F, t1210: F, t1211: F, t1215: F, t12607: F, t12622: F, t12658: F, t12696: F, t1274: F, t1277: F, t1294: F, t1295: F, t13165: F, t13166: F, t13177: F, t3556: F, t3561: F, t3567: F, t3576: F, t3585: F, t3737: F, t3738: F, t45545: F, t45552: F, t45553: F, t45559: F, t45568: F, t45575: F, t45617: F, t45652: F, t45691: F, t1298: F, t1300: F, t13190: F, t198: F, t336: F, t3801: F, t44096: F, t44100: F, t44103: F, t44106: F, t44108: F, t44111: F, t44114: F, t44122: F, t44123: F, t44126: F, t44984: F, t44987: F, t45448: F, t45494: F, t45544: F, t5023: F, t45015: F, t45021: F, t45023: F, t45026: F, t45029: F, t45033: F, t45037: F, t45040: F, t45043: F, t45045: F, t45048: F, t45050: F, t45052: F, t12587: F, t3794: F, t3798: F, t45282: F, t45296: F, t45298: F, t45302: F, t45306: F, t45310: F, t45312: F, t45316: F, t45318: F, t45321: F, t45323: F, t45326: F, t33: F, t265: F, t502: F, t41211: F, t44088: F, t10326: F, t11095: F, t1113: F, t1304: F, t13196: F, t2258: F, t2838: F, t3351: F, t3805: F, t39457: F, t43744: F, t504: F, t57: F, t606: F, t895: F, t9357: F, dens_threshold: F, rho1: F, zeta_threshold: F, t10: F, t22: F, t576: F, t588: F, t15: F, t27: F, t11: F, t10276: F, t2224: F, t584: F, t596: F, t20: F, t2237: F) -> (F, F, F, F, F, F, F, F, F) {
        let t45723 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1675::<F>(t12657, t3754, t12722, t3555, t12640, t3552, t3766, t5462, t5477, t12699, t12709, t12714, t12719, t12723, t12727, t12748, t12753, t12757, t17955, t3756, t3770, t3778, t44639, t5478, t5480);
        let (t45734, t45760) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1676::<F>(t12690, t1284, t3601, t3727, t1209, t17948, t12050, t471, t1214, t3588, t12699, t12714, t12744, t1280, t1288, t13121, t13148, t13149, t13153, t17888, t17949, t3666, t3670, t3767, t3769, t3774, t44501, t44585, t44944, t45391, t45584);
        let (t45764, t45769, t45779, t45786, t45787, t45796) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1677::<F>(t3552, t3781, t1204, t13147, t13141, t3596, t42859, t460, t3603, t43351, t1214, t17703);
        let t45800 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1678::<F>(t1248, t12629, t12646, t12712, t12717, t12734, t12741, t12751, t1287, t12975, t12987, t13144, t13150, t3302, t3746, t3759, t3760, t3767, t3769, t3782, t3783, t3784, t45609, t45648, t45734, t45764, t45769, t45779, t45786, t45787, t45796);
        let t45838 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1679::<F>(t1243, t42859, t460, t43351, t471, t1234, t1269, t12732, t1280, t1285, t1287, t12966, t12987, t13127, t13129, t13156, t3552, t3787, t44639, t44778, t44843, t44845, t44878, t45329, t45406, t45584, t45609, t487, t489, t5463, t5465);
        let t45873 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1680::<F>(t1204, t13126, t12722, t3566, t5462, t5477, t1209, t1284, t3727, t1234, t12621, t12706, t12719, t12769, t1281, t12975, t13130, t13133, t13161, t17864, t3568, t3666, t3670, t3756, t3759, t3763, t3769, t3783, t44552, t44832, t45385);
        let t45895 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1681::<F>(t1210, t1211, t1214, t1215, t12607, t12622, t12658, t12696, t1274, t1277, t1294, t1295, t13165, t13166, t13177, t3556, t3561, t3567, t3568, t3576, t3585, t3737, t3738, t44944, t45391, t45545, t45552, t45553, t45559, t45568, t45575, t45617, t45652, t45691, t45723, t45760, t45800, t45838, t45873);
        let t45901 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1682::<F>(t1298, t1300, t13190, t198, t336, t3801, t44096, t44100, t44103, t44106, t44108, t44111, t44114, t44122, t44123, t44126, t44984, t44987, t45448, t45494, t45544, t45895, t5023);
        let t45903 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1683::<F>(t45015, t45021, t45023, t45026, t45029, t45033, t45037, t45040, t45043, t45045, t45048, t45050, t45052);
        let t45908 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1684::<F>(t12587, t3794, t3798, t45282, t45296, t45298, t45302, t45306, t45310, t45312, t45316, t45318, t45321, t45323, t45326, t5023);
        let t45923 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1685::<F>(t33, t265, t502, t41211, t44088, t45901, t45903, t45908, t10326, t11095, t1113, t1304, t13196, t2258, t2838, t3351, t3805, t39457, t43744, t504, t57, t606, t895, t9357, dens_threshold, rho1, zeta_threshold);
        let (t45927, t45929, t45931, t45933, t45935, t45937, t45939, t45941) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1686::<F>(t10, t22, t576, t588, t15, t27, t11, t10276, t2224, t584, t596, t20, t2237);
    (t45923, t45927, t45929, t45931, t45933, t45935, t45937, t45939, t45941)
}
