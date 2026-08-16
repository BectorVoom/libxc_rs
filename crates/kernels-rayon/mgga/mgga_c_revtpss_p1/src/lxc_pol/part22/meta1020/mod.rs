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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1020(t1025: f64, t371: f64, t6276: f64, t676: f64, t15749: f64, t4858: f64, t11789: f64, t20016: f64, t3205: f64, t6337: f64, t15666: f64, t1053: f64, t19463: f64, t1028: f64, t11774: f64, t13396: f64, t16049: f64, t19718: f64, t20039: f64, t3092: f64, t372: f64, t373: f64, t42265: f64, t4573: f64, t53545: f64, t55002: f64, t55004: f64, t55011: f64, t6339: f64, t65122: f64, t11921: f64, t19414: f64, t247: f64, t4837: f64, t11710: f64, t20078: f64, t3091: f64, t11922: f64, t11927: f64, t19621: f64, t1045: f64, t1062: f64, t11866: f64, t15691: f64, t15728: f64, t15809: f64, t1592: f64, t15950: f64, t16089: f64, t16095: f64, t16154: f64, t19705: f64, t19838: f64, t19878: f64, t20083: f64, t20101: f64, t3075: f64, t43038: f64, t4578: f64, t4839: f64, t53885: f64, t54695: f64, t55033: f64, t6273: f64, t4787: f64, t53391: f64, t19857: f64, t1011: f64, t1042: f64, t1068: f64, t15716: f64, t16012: f64, t16152: f64, t1663: f64, t19864: f64, t42425: f64, t43066: f64, t43204: f64, t43215: f64, t55058: f64, t55061: f64, t55064: f64, t55067: f64, t55070: f64, t55072: f64, t6263: f64, t63302: f64, t15745: f64, t4845: f64, t1012: f64, t15149: f64, t15651: f64, t15656: f64, t15696: f64, t15700: f64, t15958: f64, t1665: f64, t19620: f64, t20089: f64, t3117: f64, t3236: f64, t4782: f64, t4854: f64, t53866: f64, t54384: f64, t54818: f64, t55104: f64, t55148: f64, t55150: f64, t60717: f64, t11859: f64, t20074: f64, t15926: f64, t16035: f64, t11672: f64, t12004: f64, t15154: f64, t15600: f64, t15899: f64, t19980: f64, t20079: f64, t43238: f64, t43242: f64, t54570: f64, t55152: f64, t55154: f64, t55171: f64, t6323: f64, t19830: f64, t16055: f64, t19738: f64, t20100: f64, t43131: f64, t15906: f64, t15908: f64, t16025: f64, t16081: f64, t16082: f64, t16098: f64, t16226: f64, t19611: f64, t19745: f64, t19831: f64, t20096: f64, t3155: f64, t43285: f64, t4907: f64, t54089: f64, t54916: f64, t55182: f64, t65144: f64, t66667: f64, t66766: f64, t20069: f64, t4899: f64, t11704: f64, t11875: f64, t11933: f64, t15690: f64, t15702: f64, t15782: f64, t15917: f64, t15962: f64, t16043: f64, t16170: f64, t19501: f64, t19750: f64, t19979: f64, t20070: f64, t2852: f64, t2857: f64, t3115: f64, t4823: f64, t53553: f64, t65186: f64, t65876: f64, t66341: f64, t73: f64, t20065: f64, t4892: f64, t4772: f64, t4866: f64, t15688: f64, t16584: f64, t13312: f64, t15618: f64, t15693: f64, t15959: f64, t15969: f64, t19497: f64, t20040: f64, t20105: f64, t3094: f64, t42155: f64, t43082: f64, t4781: f64, t4783: f64, t4806: f64, t54578: f64, t55233: f64, t55247: f64, t66037: f64, t15731: f64, t4879: f64, t20020: f64, t3224: f64, t127: f64, t19768: f64, t225: f64, t64686: f64, t366: f64, t11656: f64, t11783: f64, t19693: f64, t19770: f64, t19861: f64, t3162: f64, t3208: f64, t3211: f64, t43044: f64, t43050: f64, t54672: f64, t6271: f64, t6278: f64, t65261: f64, t66062: f64, t64907: f64, t19773: f64, t3215: f64, t16067: f64, t19721: f64, t19566: f64, t3090: f64, t15158: f64, t3097: f64, t3220: f64, t43121: f64, t4910: f64, t55265: f64, t55272: f64, t55279: f64, t55290: f64, t64989: f64, t66395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67186, t67195, t67199, t67206, t67213, t67215) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3538(t1025, t371, t6276, t676, t15749, t4858, t11789, t20016, t3205, t6337, t15666, t1053, t19463);
        let t67218 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3539(t1028, t11774, t13396, t16049, t19718, t20039, t3092, t3205, t371, t372, t373, t42265, t4573, t53545, t55002, t55004, t55011, t6339, t65122, t67186, t67195, t67199, t67206, t67213, t67215);
        let t67257 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3540(t11921, t19414, t247, t4837, t11710, t20078, t3091, t11922, t11927, t19621, t1045, t1062, t11774, t11866, t15691, t15728, t15809, t1592, t15950, t16089, t16095, t16154, t19705, t19838, t19878, t20083, t20101, t3075, t3092, t43038, t4578, t4839, t53885, t54695, t55033, t6273);
        let t67283 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3541(t11774, t4787, t53391, t1062, t19857, t1011, t1042, t1068, t15716, t16012, t16152, t1663, t19864, t42425, t43066, t43204, t43215, t55058, t55061, t55064, t55067, t55070, t55072, t6263, t63302);
        let t67318 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3542(t15745, t4845, t1011, t1012, t1045, t11774, t11927, t15149, t15651, t15656, t15691, t15696, t15700, t15958, t1665, t19620, t20089, t3117, t3236, t4782, t4854, t4858, t53866, t54384, t54818, t55104, t55148, t55150, t60717);
        let t67345 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3543(t11859, t11922, t20074, t15926, t16035, t1045, t11672, t11774, t12004, t15154, t15600, t15696, t15700, t15899, t19980, t20079, t43238, t43242, t4787, t54570, t54818, t55152, t55154, t55171, t6323);
        let t67382 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3544(t11922, t11927, t19830, t16055, t19738, t16095, t20100, t43131, t11866, t15691, t15906, t15908, t16025, t16081, t16082, t16098, t16226, t19611, t19745, t19831, t20096, t3117, t3155, t43285, t4907, t54089, t54916, t55182, t65144, t66667, t66766);
        let t67430 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3545(t11922, t20069, t4899, t1045, t11704, t11774, t11859, t11875, t11933, t15690, t15702, t15782, t15917, t15962, t16043, t16049, t16170, t19501, t19718, t19738, t19745, t19750, t19979, t20070, t2852, t2857, t3115, t3117, t3155, t372, t4823, t53553, t65186, t65876, t66341, t73);
        let (t67438, t67470) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3546(t11922, t20065, t4892, t4772, t4866, t15688, t16584, t1042, t1045, t11927, t11933, t13312, t15618, t15693, t15696, t15959, t15969, t19497, t19620, t20040, t20105, t3091, t3092, t3094, t3115, t3117, t42155, t43082, t4781, t4783, t4806, t4837, t54578, t55233, t55247, t66037);
        let (t67501, t67509) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3547(t15731, t4879, t20020, t3224, t1025, t127, t19768, t371, t225, t64686, t366, t1045, t11656, t11783, t11927, t15700, t19693, t19770, t19861, t3075, t3117, t3155, t3162, t3208, t3211, t42155, t43044, t43050, t54672, t6271, t6278, t65261, t66062);
        let t67543 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3548(t366, t64907, t19773, t3215, t11922, t16067, t19721, t19566, t3090, t1025, t1028, t1045, t15158, t15691, t15700, t3097, t3115, t3117, t3220, t371, t372, t373, t43121, t4910, t55265, t55272, t55279, t55290, t6273, t64989, t66395);
    (t67218, t67257, t67283, t67318, t67345, t67382, t67430, t67438, t67470, t67501, t67509, t67543)
}
