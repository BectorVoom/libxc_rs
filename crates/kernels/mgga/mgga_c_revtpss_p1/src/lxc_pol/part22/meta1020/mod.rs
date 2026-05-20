//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1020 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3538;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3539;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3540;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3541;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3542;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3543;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3544;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3545;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3546;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3547;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1020<F: Float>(t1025: F, t371: F, t6276: F, t676: F, t15749: F, t4858: F, t11789: F, t20016: F, t3205: F, t6337: F, t15666: F, t1053: F, t19463: F, t1028: F, t11774: F, t13396: F, t16049: F, t19718: F, t20039: F, t3092: F, t372: F, t373: F, t42265: F, t4573: F, t53545: F, t55002: F, t55004: F, t55011: F, t6339: F, t65122: F, t11921: F, t19414: F, t247: F, t4837: F, t11710: F, t20078: F, t3091: F, t11922: F, t11927: F, t19621: F, t1045: F, t1062: F, t11866: F, t15691: F, t15728: F, t15809: F, t1592: F, t15950: F, t16089: F, t16095: F, t16154: F, t19705: F, t19838: F, t19878: F, t20083: F, t20101: F, t3075: F, t43038: F, t4578: F, t4839: F, t53885: F, t54695: F, t55033: F, t6273: F, t4787: F, t53391: F, t19857: F, t1011: F, t1042: F, t1068: F, t15716: F, t16012: F, t16152: F, t1663: F, t19864: F, t42425: F, t43066: F, t43204: F, t43215: F, t55058: F, t55061: F, t55064: F, t55067: F, t55070: F, t55072: F, t6263: F, t63302: F, t15745: F, t4845: F, t1012: F, t15149: F, t15651: F, t15656: F, t15696: F, t15700: F, t15958: F, t1665: F, t19620: F, t20089: F, t3117: F, t3236: F, t4782: F, t4854: F, t53866: F, t54384: F, t54818: F, t55104: F, t55148: F, t55150: F, t60717: F, t11859: F, t20074: F, t15926: F, t16035: F, t11672: F, t12004: F, t15154: F, t15600: F, t15899: F, t19980: F, t20079: F, t43238: F, t43242: F, t54570: F, t55152: F, t55154: F, t55171: F, t6323: F, t19830: F, t16055: F, t19738: F, t20100: F, t43131: F, t15906: F, t15908: F, t16025: F, t16081: F, t16082: F, t16098: F, t16226: F, t19611: F, t19745: F, t19831: F, t20096: F, t3155: F, t43285: F, t4907: F, t54089: F, t54916: F, t55182: F, t65144: F, t66667: F, t66766: F, t20069: F, t4899: F, t11704: F, t11875: F, t11933: F, t15690: F, t15702: F, t15782: F, t15917: F, t15962: F, t16043: F, t16170: F, t19501: F, t19750: F, t19979: F, t20070: F, t2852: F, t2857: F, t3115: F, t4823: F, t53553: F, t65186: F, t65876: F, t66341: F, t73: F, t20065: F, t4892: F, t4772: F, t4866: F, t15688: F, t16584: F, t13312: F, t15618: F, t15693: F, t15959: F, t15969: F, t19497: F, t20040: F, t20105: F, t3094: F, t42155: F, t43082: F, t4781: F, t4783: F, t4806: F, t54578: F, t55233: F, t55247: F, t66037: F, t15731: F, t4879: F, t20020: F, t3224: F, t127: F, t19768: F, t225: F, t64686: F, t366: F, t11656: F, t11783: F, t19693: F, t19770: F, t19861: F, t3162: F, t3208: F, t3211: F, t43044: F, t43050: F, t54672: F, t6271: F, t6278: F, t65261: F, t66062: F, t64907: F, t19773: F, t3215: F, t16067: F, t19721: F, t19566: F, t3090: F, t15158: F, t3097: F, t3220: F, t43121: F, t4910: F, t55265: F, t55272: F, t55279: F, t55290: F, t64989: F, t66395: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t67186, t67195, t67199, t67206, t67213, t67215) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3538::<F>(t1025, t371, t6276, t676, t15749, t4858, t11789, t20016, t3205, t6337, t15666, t1053, t19463);
        let t67218 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3539::<F>(t1028, t11774, t13396, t16049, t19718, t20039, t3092, t3205, t371, t372, t373, t42265, t4573, t53545, t55002, t55004, t55011, t6339, t65122, t67186, t67195, t67199, t67206, t67213, t67215);
        let t67257 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3540::<F>(t11921, t19414, t247, t4837, t11710, t20078, t3091, t11922, t11927, t19621, t1045, t1062, t11774, t11866, t15691, t15728, t15809, t1592, t15950, t16089, t16095, t16154, t19705, t19838, t19878, t20083, t20101, t3075, t3092, t43038, t4578, t4839, t53885, t54695, t55033, t6273);
        let t67283 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3541::<F>(t11774, t4787, t53391, t1062, t19857, t1011, t1042, t1068, t15716, t16012, t16152, t1663, t19864, t42425, t43066, t43204, t43215, t55058, t55061, t55064, t55067, t55070, t55072, t6263, t63302);
        let t67318 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3542::<F>(t15745, t4845, t1011, t1012, t1045, t11774, t11927, t15149, t15651, t15656, t15691, t15696, t15700, t15958, t1665, t19620, t20089, t3117, t3236, t4782, t4854, t4858, t53866, t54384, t54818, t55104, t55148, t55150, t60717);
        let t67345 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3543::<F>(t11859, t11922, t20074, t15926, t16035, t1045, t11672, t11774, t12004, t15154, t15600, t15696, t15700, t15899, t19980, t20079, t43238, t43242, t4787, t54570, t54818, t55152, t55154, t55171, t6323);
        let t67382 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3544::<F>(t11922, t11927, t19830, t16055, t19738, t16095, t20100, t43131, t11866, t15691, t15906, t15908, t16025, t16081, t16082, t16098, t16226, t19611, t19745, t19831, t20096, t3117, t3155, t43285, t4907, t54089, t54916, t55182, t65144, t66667, t66766);
        let t67430 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3545::<F>(t11922, t20069, t4899, t1045, t11704, t11774, t11859, t11875, t11933, t15690, t15702, t15782, t15917, t15962, t16043, t16049, t16170, t19501, t19718, t19738, t19745, t19750, t19979, t20070, t2852, t2857, t3115, t3117, t3155, t372, t4823, t53553, t65186, t65876, t66341, t73);
        let (t67438, t67470) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3546::<F>(t11922, t20065, t4892, t4772, t4866, t15688, t16584, t1042, t1045, t11927, t11933, t13312, t15618, t15693, t15696, t15959, t15969, t19497, t19620, t20040, t20105, t3091, t3092, t3094, t3115, t3117, t42155, t43082, t4781, t4783, t4806, t4837, t54578, t55233, t55247, t66037);
        let (t67501, t67509) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3547::<F>(t15731, t4879, t20020, t3224, t1025, t127, t19768, t371, t225, t64686, t366, t1045, t11656, t11783, t11927, t15700, t19693, t19770, t19861, t3075, t3117, t3155, t3162, t3208, t3211, t42155, t43044, t43050, t54672, t6271, t6278, t65261, t66062);
        let t67543 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3548::<F>(t366, t64907, t19773, t3215, t11922, t16067, t19721, t19566, t3090, t1025, t1028, t1045, t15158, t15691, t15700, t3097, t3115, t3117, t3220, t371, t372, t373, t43121, t4910, t55265, t55272, t55279, t55290, t6273, t64989, t66395);
    (t67218, t67257, t67283, t67318, t67345, t67382, t67430, t67438, t67470, t67501, t67509, t67543)
}
