//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta925 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2996;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2997;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2998;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2999;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3000;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3001;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta925<F: Float>(t1469: F, t1668: F, t66066: F, t19634: F, t78900: F, t11774: F, t53391: F, t6267: F, t23598: F, t999: F, t19380: F, t4866: F, t6258: F, t1045: F, t11866: F, t15700: F, t15701: F, t15926: F, t16089: F, t16226: F, t20038: F, t20040: F, t20105: F, t23964: F, t23980: F, t23994: F, t247: F, t3092: F, t3106: F, t3115: F, t3116: F, t3117: F, t3155: F, t3162: F, t42328: F, t4579: F, t4837: F, t4900: F, t53676: F, t54079: F, t54818: F, t55141: F, t66187: F, t66328: F, t66332: F, t78812: F, t906: F, t19968: F, t4817: F, t20054: F, t4834: F, t11631: F, t11661: F, t11859: F, t15906: F, t16081: F, t19450: F, t19572: F, t19861: F, t23485: F, t23929: F, t3091: F, t4786: F, t54118: F, t54123: F, t54127: F, t54696: F, t6339: F, t66355: F, t66362: F, t66376: F, t66403: F, t66406: F, t66423: F, t66467: F, t66470: F, t78496: F, t19882: F, t1062: F, t23960: F, t11921: F, t11246: F, t23833: F, t3172: F, t1063: F, t23851: F, t1042: F, t1068: F, t15817: F, t15850: F, t19800: F, t23834: F, t23852: F, t23886: F, t3188: F, t42648: F, t42716: F, t42740: F, t42745: F, t4879: F, t54148: F, t54537: F, t6302: F, t6331: F, t66547: F, t66551: F, t78785: F, t19639: F, t1043: F, t15689: F, t16052: F, t19864: F, t19982: F, t19986: F, t19992: F, t19997: F, t19998: F, t23931: F, t42781: F, t42872: F, t43069: F, t4787: F, t54388: F, t54414: F, t54509: F, t66114: F, t66306: F, t66542: F, t66644: F, t66647: F, t66655: F, t66660: F, t66686: F, t66777: F, t67052: F, t67458: F, t1011: F, t140: F, t23873: F, t1012: F, t1015: F, t15584: F, t15917: F, t19620: F, t19718: F, t19741: F, t19754: F, t23837: F, t23936: F, t43044: F, t43050: F, t4783: F, t66624: F, t66712: F, t66714: F, t66721: F, t66731: F, t66739: F, t66747: F, t66752: F, t66758: F, t66763: F, t67528: F, t76397: F, t78884: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t79463, t79467, t79474, t79480, t79500, t79505) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2996::<F>(t1469, t1668, t66066, t19634, t78900, t11774, t53391, t6267, t23598, t999, t19380, t4866, t6258);
        let t79514 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2997::<F>(t1045, t11774, t11866, t15700, t15701, t15926, t16089, t16226, t20038, t20040, t20105, t23964, t23980, t23994, t247, t3092, t3106, t3115, t3116, t3117, t3155, t3162, t42328, t4579, t4837, t4900, t53676, t54079, t54818, t55141, t6267, t66187, t66328, t66332, t78812, t79463, t79467, t79474, t79480, t79500, t79505, t906);
        let t79550 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2998::<F>(t19968, t4817, t20054, t4834, t11631, t11661, t11859, t15906, t16081, t19450, t19572, t19861, t23485, t23929, t3091, t3092, t3117, t4786, t4866, t54118, t54123, t54127, t54696, t55141, t6339, t66355, t66362, t66376, t66403, t66406, t66423, t66467, t66470, t78496, t999);
        let (t79553, t79559, t79564, t79575, t79580) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2999::<F>(t19882, t4834, t1062, t23960, t11921, t23964, t247, t4837, t11246, t23833, t3172, t1063, t23851);
        let t79588 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3000::<F>(t1042, t1063, t1068, t15817, t15850, t19800, t23834, t23852, t23886, t3106, t3188, t42648, t42716, t42740, t42745, t4879, t54148, t54537, t6302, t6331, t66547, t66551, t78785, t79553, t79559, t79564, t79575, t79580);
        let (t79610, t79627) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3001::<F>(t19639, t78900, t1043, t11774, t15689, t15700, t15701, t16052, t16226, t19864, t19982, t19986, t19992, t19997, t19998, t23931, t3117, t42781, t42872, t43069, t4787, t54388, t54414, t54509, t55141, t66114, t66306, t66542, t66644, t66647, t66655, t66660, t66686, t66777, t67052, t67458, t78812);
        let t79665 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3002::<F>(t1011, t140, t23873, t1012, t1015, t11774, t15584, t15917, t19620, t19634, t19639, t19718, t19741, t19754, t23837, t23936, t3117, t43044, t43050, t4783, t66624, t66712, t66714, t66721, t66731, t66739, t66747, t66752, t66758, t66763, t67528, t76397, t78884);
    (t79463, t79467, t79480, t79500, t79505, t79514, t79550, t79588, t79610, t79627, t79665)
}
