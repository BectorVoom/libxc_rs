//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1019 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3531;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3532;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3533;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3534;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3535;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3536;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1019<F: Float>(t11859: F, t11922: F, t19635: F, t1043: F, t19971: F, t11875: F, t19640: F, t11675: F, t15622: F, t15906: F, t15907: F, t15963: F, t19611: F, t19778: F, t19873: F, t20096: F, t3091: F, t3092: F, t3117: F, t42417: F, t42996: F, t53885: F, t54500: F, t54785: F, t54792: F, t54795: F, t6268: F, t6244: F, t905: F, t11774: F, t4782: F, t53391: F, t1011: F, t15993: F, t18909: F, t11792: F, t15599: F, t15610: F, t15688: F, t15696: F, t15964: F, t15968: F, t16096: F, t16103: F, t16183: F, t1664: F, t3154: F, t43049: F, t4892: F, t4893: F, t54836: F, t54841: F, t54849: F, t54869: F, t55331: F, t6096: F, t6278: F, t66777: F, t11933: F, t19976: F, t3115: F, t42793: F, t6272: F, t16081: F, t19749: F, t11866: F, t15758: F, t15917: F, t15922: F, t15926: F, t16052: F, t16078: F, t19726: F, t19758: F, t20066: F, t20070: F, t20075: F, t20105: F, t42643: F, t42830: F, t4891: F, t4896: F, t4907: F, t53855: F, t55958: F, t20020: F, t3211: F, t15656: F, t4845: F, t19675: F, t372: F, t11779: F, t15703: F, t15745: F, t16067: F, t16068: F, t1665: F, t20091: F, t3096: F, t4854: F, t54699: F, t54907: F, t54914: F, t54919: F, t54925: F, t65144: F, t66542: F, t11947: F, t20016: F, t19620: F, t66061: F, t1045: F, t11696: F, t11703: F, t11705: F, t15618: F, t15691: F, t15700: F, t1592: F, t15965: F, t16089: F, t16222: F, t16226: F, t19501: F, t19981: F, t19997: F, t20099: F, t3059: F, t3181: F, t42360: F, t43069: F, t43151: F, t4866: F, t4899: F, t53545: F, t54943: F, t6339: F, t65876: F, t19680: F, t4786: F, t11660: F, t15584: F, t15689: F, t15701: F, t16040: F, t19622: F, t19700: F, t19985: F, t19992: F, t20040: F, t42695: F, t43066: F, t43082: F, t43285: F, t4583: F, t53585: F, t54991: F, t55209: F, t6092: F, t6273: F, t999: F, t19757: F, t11144: F, t15586: F, t15595: F, t15936: F, t16049: F, t16095: F, t1651: F, t19626: F, t19864: F, t20038: F, t42155: F, t42328: F, t42410: F, t4574: F, t4900: F, t54994: F, t55000: F, t55122: F, t55141: F, t65060: F, t66734: F) -> (F, F, F, F, F, F, F, F) {
        let (t66945, t66956) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3531::<F>(t11859, t11922, t19635, t1043, t19971, t11875, t19640, t11675, t15622, t15906, t15907, t15963, t19611, t19778, t19873, t20096, t3091, t3092, t3117, t42417, t42996, t53885, t54500, t54785, t54792, t54795, t6268);
        let t66997 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3532::<F>(t6244, t905, t11774, t4782, t53391, t1011, t15993, t18909, t11792, t15599, t15610, t15688, t15696, t15964, t15968, t16096, t16103, t16183, t1664, t3091, t3092, t3117, t3154, t43049, t4892, t4893, t54836, t54841, t54849, t54869, t55331, t6096, t6278, t66777);
        let t67031 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3533::<F>(t11933, t19976, t3115, t42793, t6272, t11922, t16081, t19749, t11866, t15758, t15917, t15922, t15926, t16052, t16078, t19726, t19758, t20066, t20070, t20075, t20105, t42643, t42830, t4891, t4896, t4907, t53855, t55958);
        let t67058 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3534::<F>(t20020, t3211, t15656, t4845, t19675, t372, t11774, t11779, t11933, t15703, t15745, t16067, t16068, t1665, t20091, t3096, t3117, t4854, t54699, t54907, t54914, t54919, t54925, t6278, t65144, t66542);
        let (t67090, t67102) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3535::<F>(t11947, t20016, t19620, t66061, t1045, t11696, t11703, t11705, t11774, t15618, t15691, t15700, t1592, t15965, t16089, t16222, t16226, t19501, t19611, t19981, t19997, t20099, t3059, t3091, t3092, t3181, t372, t42360, t43069, t43151, t4866, t4899, t53545, t54943, t6339, t65876);
        let t67143 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3536::<F>(t19680, t4786, t1045, t11660, t11703, t11774, t15584, t15689, t15691, t15700, t15701, t15926, t15968, t16040, t16222, t19622, t19700, t19985, t19992, t20040, t42695, t43066, t43082, t43285, t4583, t4892, t53545, t53585, t54991, t55209, t6092, t6273, t999);
        let t67182 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3537::<F>(t11875, t11922, t19757, t1045, t11144, t11774, t11866, t15586, t15595, t15689, t15701, t15936, t16049, t16095, t1651, t19626, t19864, t20038, t20091, t3115, t3117, t42155, t42328, t42410, t4574, t4900, t54994, t55000, t55122, t55141, t55209, t65060, t66734, t67090);
    (t66945, t66956, t66997, t67031, t67058, t67102, t67143, t67182)
}
