//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta925 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2996;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2997;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2998;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2999;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3000;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3001;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta925(t1469: f64, t1668: f64, t66066: f64, t19634: f64, t78900: f64, t11774: f64, t53391: f64, t6267: f64, t23598: f64, t999: f64, t19380: f64, t4866: f64, t6258: f64, t1045: f64, t11866: f64, t15700: f64, t15701: f64, t15926: f64, t16089: f64, t16226: f64, t20038: f64, t20040: f64, t20105: f64, t23964: f64, t23980: f64, t23994: f64, t247: f64, t3092: f64, t3106: f64, t3115: f64, t3116: f64, t3117: f64, t3155: f64, t3162: f64, t42328: f64, t4579: f64, t4837: f64, t4900: f64, t53676: f64, t54079: f64, t54818: f64, t55141: f64, t66187: f64, t66328: f64, t66332: f64, t78812: f64, t906: f64, t19968: f64, t4817: f64, t20054: f64, t4834: f64, t11631: f64, t11661: f64, t11859: f64, t15906: f64, t16081: f64, t19450: f64, t19572: f64, t19861: f64, t23485: f64, t23929: f64, t3091: f64, t4786: f64, t54118: f64, t54123: f64, t54127: f64, t54696: f64, t6339: f64, t66355: f64, t66362: f64, t66376: f64, t66403: f64, t66406: f64, t66423: f64, t66467: f64, t66470: f64, t78496: f64, t19882: f64, t1062: f64, t23960: f64, t11921: f64, t11246: f64, t23833: f64, t3172: f64, t1063: f64, t23851: f64, t1042: f64, t1068: f64, t15817: f64, t15850: f64, t19800: f64, t23834: f64, t23852: f64, t23886: f64, t3188: f64, t42648: f64, t42716: f64, t42740: f64, t42745: f64, t4879: f64, t54148: f64, t54537: f64, t6302: f64, t6331: f64, t66547: f64, t66551: f64, t78785: f64, t19639: f64, t1043: f64, t15689: f64, t16052: f64, t19864: f64, t19982: f64, t19986: f64, t19992: f64, t19997: f64, t19998: f64, t23931: f64, t42781: f64, t42872: f64, t43069: f64, t4787: f64, t54388: f64, t54414: f64, t54509: f64, t66114: f64, t66306: f64, t66542: f64, t66644: f64, t66647: f64, t66655: f64, t66660: f64, t66686: f64, t66777: f64, t67052: f64, t67458: f64, t1011: f64, t140: f64, t23873: f64, t1012: f64, t1015: f64, t15584: f64, t15917: f64, t19620: f64, t19718: f64, t19741: f64, t19754: f64, t23837: f64, t23936: f64, t43044: f64, t43050: f64, t4783: f64, t66624: f64, t66712: f64, t66714: f64, t66721: f64, t66731: f64, t66739: f64, t66747: f64, t66752: f64, t66758: f64, t66763: f64, t67528: f64, t76397: f64, t78884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79463, t79467, t79474, t79480, t79500, t79505) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2996(t1469, t1668, t66066, t19634, t78900, t11774, t53391, t6267, t23598, t999, t19380, t4866, t6258);
        let t79514 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2997(t1045, t11774, t11866, t15700, t15701, t15926, t16089, t16226, t20038, t20040, t20105, t23964, t23980, t23994, t247, t3092, t3106, t3115, t3116, t3117, t3155, t3162, t42328, t4579, t4837, t4900, t53676, t54079, t54818, t55141, t6267, t66187, t66328, t66332, t78812, t79463, t79467, t79474, t79480, t79500, t79505, t906);
        let t79550 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2998(t19968, t4817, t20054, t4834, t11631, t11661, t11859, t15906, t16081, t19450, t19572, t19861, t23485, t23929, t3091, t3092, t3117, t4786, t4866, t54118, t54123, t54127, t54696, t55141, t6339, t66355, t66362, t66376, t66403, t66406, t66423, t66467, t66470, t78496, t999);
        let (t79553, t79559, t79564, t79575, t79580) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2999(t19882, t4834, t1062, t23960, t11921, t23964, t247, t4837, t11246, t23833, t3172, t1063, t23851);
        let t79588 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3000(t1042, t1063, t1068, t15817, t15850, t19800, t23834, t23852, t23886, t3106, t3188, t42648, t42716, t42740, t42745, t4879, t54148, t54537, t6302, t6331, t66547, t66551, t78785, t79553, t79559, t79564, t79575, t79580);
        let (t79610, t79627) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3001(t19639, t78900, t1043, t11774, t15689, t15700, t15701, t16052, t16226, t19864, t19982, t19986, t19992, t19997, t19998, t23931, t3117, t42781, t42872, t43069, t4787, t54388, t54414, t54509, t55141, t66114, t66306, t66542, t66644, t66647, t66655, t66660, t66686, t66777, t67052, t67458, t78812);
        let t79665 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3002(t1011, t140, t23873, t1012, t1015, t11774, t15584, t15917, t19620, t19634, t19639, t19718, t19741, t19754, t23837, t23936, t3117, t43044, t43050, t4783, t66624, t66712, t66714, t66721, t66731, t66739, t66747, t66752, t66758, t66763, t67528, t76397, t78884);
    (t79463, t79467, t79480, t79500, t79505, t79514, t79550, t79588, t79610, t79627, t79665)
}
