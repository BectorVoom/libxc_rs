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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta441(t12657: f64, t3754: f64, t12722: f64, t3555: f64, t12640: f64, t3552: f64, t3766: f64, t5462: f64, t5477: f64, t12699: f64, t12709: f64, t12714: f64, t12719: f64, t12723: f64, t12727: f64, t12748: f64, t12753: f64, t12757: f64, t17955: f64, t3756: f64, t3770: f64, t3778: f64, t44639: f64, t5478: f64, t5480: f64, t12690: f64, t1284: f64, t3601: f64, t3727: f64, t1209: f64, t17948: f64, t12050: f64, t471: f64, t1214: f64, t3588: f64, t12744: f64, t1280: f64, t1288: f64, t13121: f64, t13148: f64, t13149: f64, t13153: f64, t17888: f64, t17949: f64, t3666: f64, t3670: f64, t3767: f64, t3769: f64, t3774: f64, t44501: f64, t44585: f64, t44944: f64, t45391: f64, t45584: f64, t3781: f64, t1204: f64, t13147: f64, t13141: f64, t3596: f64, t42859: f64, t460: f64, t3603: f64, t43351: f64, t17703: f64, t1248: f64, t12629: f64, t12646: f64, t12712: f64, t12717: f64, t12734: f64, t12741: f64, t12751: f64, t1287: f64, t12975: f64, t12987: f64, t13144: f64, t13150: f64, t3302: f64, t3746: f64, t3759: f64, t3760: f64, t3782: f64, t3783: f64, t3784: f64, t45609: f64, t45648: f64, t1243: f64, t1234: f64, t1269: f64, t12732: f64, t1285: f64, t12966: f64, t13127: f64, t13129: f64, t13156: f64, t3787: f64, t44778: f64, t44843: f64, t44845: f64, t44878: f64, t45329: f64, t45406: f64, t487: f64, t489: f64, t5463: f64, t5465: f64, t13126: f64, t3566: f64, t12621: f64, t12706: f64, t12769: f64, t1281: f64, t13130: f64, t13133: f64, t13161: f64, t17864: f64, t3568: f64, t3763: f64, t44552: f64, t44832: f64, t45385: f64, t1210: f64, t1211: f64, t1215: f64, t12607: f64, t12622: f64, t12658: f64, t12696: f64, t1274: f64, t1277: f64, t1294: f64, t1295: f64, t13165: f64, t13166: f64, t13177: f64, t3556: f64, t3561: f64, t3567: f64, t3576: f64, t3585: f64, t3737: f64, t3738: f64, t45545: f64, t45552: f64, t45553: f64, t45559: f64, t45568: f64, t45575: f64, t45617: f64, t45652: f64, t45691: f64, t1298: f64, t1300: f64, t13190: f64, t198: f64, t336: f64, t3801: f64, t44096: f64, t44100: f64, t44103: f64, t44106: f64, t44108: f64, t44111: f64, t44114: f64, t44122: f64, t44123: f64, t44126: f64, t44984: f64, t44987: f64, t45448: f64, t45494: f64, t45544: f64, t5023: f64, t45015: f64, t45021: f64, t45023: f64, t45026: f64, t45029: f64, t45033: f64, t45037: f64, t45040: f64, t45043: f64, t45045: f64, t45048: f64, t45050: f64, t45052: f64, t12587: f64, t3794: f64, t3798: f64, t45282: f64, t45296: f64, t45298: f64, t45302: f64, t45306: f64, t45310: f64, t45312: f64, t45316: f64, t45318: f64, t45321: f64, t45323: f64, t45326: f64, t33: f64, t265: f64, t502: f64, t41211: f64, t44088: f64, t10326: f64, t11095: f64, t1113: f64, t1304: f64, t13196: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t39457: f64, t43744: f64, t504: f64, t57: f64, t606: f64, t895: f64, t9357: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t10: f64, t22: f64, t576: f64, t588: f64, t15: f64, t27: f64, t11: f64, t10276: f64, t2224: f64, t584: f64, t596: f64, t20: f64, t2237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t45723 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1675(t12657, t3754, t12722, t3555, t12640, t3552, t3766, t5462, t5477, t12699, t12709, t12714, t12719, t12723, t12727, t12748, t12753, t12757, t17955, t3756, t3770, t3778, t44639, t5478, t5480);
        let (t45734, t45760) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1676(t12690, t1284, t3601, t3727, t1209, t17948, t12050, t471, t1214, t3588, t12699, t12714, t12744, t1280, t1288, t13121, t13148, t13149, t13153, t17888, t17949, t3666, t3670, t3767, t3769, t3774, t44501, t44585, t44944, t45391, t45584);
        let (t45764, t45769, t45779, t45786, t45787, t45796) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1677(t3552, t3781, t1204, t13147, t13141, t3596, t42859, t460, t3603, t43351, t1214, t17703);
        let t45800 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1678(t1248, t12629, t12646, t12712, t12717, t12734, t12741, t12751, t1287, t12975, t12987, t13144, t13150, t3302, t3746, t3759, t3760, t3767, t3769, t3782, t3783, t3784, t45609, t45648, t45734, t45764, t45769, t45779, t45786, t45787, t45796);
        let t45838 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1679(t1243, t42859, t460, t43351, t471, t1234, t1269, t12732, t1280, t1285, t1287, t12966, t12987, t13127, t13129, t13156, t3552, t3787, t44639, t44778, t44843, t44845, t44878, t45329, t45406, t45584, t45609, t487, t489, t5463, t5465);
        let t45873 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1680(t1204, t13126, t12722, t3566, t5462, t5477, t1209, t1284, t3727, t1234, t12621, t12706, t12719, t12769, t1281, t12975, t13130, t13133, t13161, t17864, t3568, t3666, t3670, t3756, t3759, t3763, t3769, t3783, t44552, t44832, t45385);
        let t45895 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1681(t1210, t1211, t1214, t1215, t12607, t12622, t12658, t12696, t1274, t1277, t1294, t1295, t13165, t13166, t13177, t3556, t3561, t3567, t3568, t3576, t3585, t3737, t3738, t44944, t45391, t45545, t45552, t45553, t45559, t45568, t45575, t45617, t45652, t45691, t45723, t45760, t45800, t45838, t45873);
        let t45901 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1682(t1298, t1300, t13190, t198, t336, t3801, t44096, t44100, t44103, t44106, t44108, t44111, t44114, t44122, t44123, t44126, t44984, t44987, t45448, t45494, t45544, t45895, t5023);
        let t45903 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1683(t45015, t45021, t45023, t45026, t45029, t45033, t45037, t45040, t45043, t45045, t45048, t45050, t45052);
        let t45908 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1684(t12587, t3794, t3798, t45282, t45296, t45298, t45302, t45306, t45310, t45312, t45316, t45318, t45321, t45323, t45326, t5023);
        let t45923 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1685(t33, t265, t502, t41211, t44088, t45901, t45903, t45908, t10326, t11095, t1113, t1304, t13196, t2258, t2838, t3351, t3805, t39457, t43744, t504, t57, t606, t895, t9357, dens_threshold, rho1, zeta_threshold);
        let (t45927, t45929, t45931, t45933, t45935, t45937, t45939, t45941) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1686(t10, t22, t576, t588, t15, t27, t11, t10276, t2224, t584, t596, t20, t2237);
    (t45923, t45927, t45929, t45931, t45933, t45935, t45937, t45939, t45941)
}
