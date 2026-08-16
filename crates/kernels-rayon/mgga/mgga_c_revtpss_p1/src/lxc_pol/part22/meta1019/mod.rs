//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1019 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3531;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3532;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3533;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3534;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3535;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3536;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1019(t11859: f64, t11922: f64, t19635: f64, t1043: f64, t19971: f64, t11875: f64, t19640: f64, t11675: f64, t15622: f64, t15906: f64, t15907: f64, t15963: f64, t19611: f64, t19778: f64, t19873: f64, t20096: f64, t3091: f64, t3092: f64, t3117: f64, t42417: f64, t42996: f64, t53885: f64, t54500: f64, t54785: f64, t54792: f64, t54795: f64, t6268: f64, t6244: f64, t905: f64, t11774: f64, t4782: f64, t53391: f64, t1011: f64, t15993: f64, t18909: f64, t11792: f64, t15599: f64, t15610: f64, t15688: f64, t15696: f64, t15964: f64, t15968: f64, t16096: f64, t16103: f64, t16183: f64, t1664: f64, t3154: f64, t43049: f64, t4892: f64, t4893: f64, t54836: f64, t54841: f64, t54849: f64, t54869: f64, t55331: f64, t6096: f64, t6278: f64, t66777: f64, t11933: f64, t19976: f64, t3115: f64, t42793: f64, t6272: f64, t16081: f64, t19749: f64, t11866: f64, t15758: f64, t15917: f64, t15922: f64, t15926: f64, t16052: f64, t16078: f64, t19726: f64, t19758: f64, t20066: f64, t20070: f64, t20075: f64, t20105: f64, t42643: f64, t42830: f64, t4891: f64, t4896: f64, t4907: f64, t53855: f64, t55958: f64, t20020: f64, t3211: f64, t15656: f64, t4845: f64, t19675: f64, t372: f64, t11779: f64, t15703: f64, t15745: f64, t16067: f64, t16068: f64, t1665: f64, t20091: f64, t3096: f64, t4854: f64, t54699: f64, t54907: f64, t54914: f64, t54919: f64, t54925: f64, t65144: f64, t66542: f64, t11947: f64, t20016: f64, t19620: f64, t66061: f64, t1045: f64, t11696: f64, t11703: f64, t11705: f64, t15618: f64, t15691: f64, t15700: f64, t1592: f64, t15965: f64, t16089: f64, t16222: f64, t16226: f64, t19501: f64, t19981: f64, t19997: f64, t20099: f64, t3059: f64, t3181: f64, t42360: f64, t43069: f64, t43151: f64, t4866: f64, t4899: f64, t53545: f64, t54943: f64, t6339: f64, t65876: f64, t19680: f64, t4786: f64, t11660: f64, t15584: f64, t15689: f64, t15701: f64, t16040: f64, t19622: f64, t19700: f64, t19985: f64, t19992: f64, t20040: f64, t42695: f64, t43066: f64, t43082: f64, t43285: f64, t4583: f64, t53585: f64, t54991: f64, t55209: f64, t6092: f64, t6273: f64, t999: f64, t19757: f64, t11144: f64, t15586: f64, t15595: f64, t15936: f64, t16049: f64, t16095: f64, t1651: f64, t19626: f64, t19864: f64, t20038: f64, t42155: f64, t42328: f64, t42410: f64, t4574: f64, t4900: f64, t54994: f64, t55000: f64, t55122: f64, t55141: f64, t65060: f64, t66734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66945, t66956) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3531(t11859, t11922, t19635, t1043, t19971, t11875, t19640, t11675, t15622, t15906, t15907, t15963, t19611, t19778, t19873, t20096, t3091, t3092, t3117, t42417, t42996, t53885, t54500, t54785, t54792, t54795, t6268);
        let t66997 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3532(t6244, t905, t11774, t4782, t53391, t1011, t15993, t18909, t11792, t15599, t15610, t15688, t15696, t15964, t15968, t16096, t16103, t16183, t1664, t3091, t3092, t3117, t3154, t43049, t4892, t4893, t54836, t54841, t54849, t54869, t55331, t6096, t6278, t66777);
        let t67031 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3533(t11933, t19976, t3115, t42793, t6272, t11922, t16081, t19749, t11866, t15758, t15917, t15922, t15926, t16052, t16078, t19726, t19758, t20066, t20070, t20075, t20105, t42643, t42830, t4891, t4896, t4907, t53855, t55958);
        let t67058 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3534(t20020, t3211, t15656, t4845, t19675, t372, t11774, t11779, t11933, t15703, t15745, t16067, t16068, t1665, t20091, t3096, t3117, t4854, t54699, t54907, t54914, t54919, t54925, t6278, t65144, t66542);
        let (t67090, t67102) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3535(t11947, t20016, t19620, t66061, t1045, t11696, t11703, t11705, t11774, t15618, t15691, t15700, t1592, t15965, t16089, t16222, t16226, t19501, t19611, t19981, t19997, t20099, t3059, t3091, t3092, t3181, t372, t42360, t43069, t43151, t4866, t4899, t53545, t54943, t6339, t65876);
        let t67143 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3536(t19680, t4786, t1045, t11660, t11703, t11774, t15584, t15689, t15691, t15700, t15701, t15926, t15968, t16040, t16222, t19622, t19700, t19985, t19992, t20040, t42695, t43066, t43082, t43285, t4583, t4892, t53545, t53585, t54991, t55209, t6092, t6273, t999);
        let t67182 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3537(t11875, t11922, t19757, t1045, t11144, t11774, t11866, t15586, t15595, t15689, t15701, t15936, t16049, t16095, t1651, t19626, t19864, t20038, t20091, t3115, t3117, t42155, t42328, t42410, t4574, t4900, t54994, t55000, t55122, t55141, t55209, t65060, t66734, t67090);
    (t66945, t66956, t66997, t67031, t67058, t67102, t67143, t67182)
}
