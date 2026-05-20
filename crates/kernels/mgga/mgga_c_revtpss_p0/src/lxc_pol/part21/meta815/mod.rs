//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta815 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2985;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2986;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2987;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2988;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2989;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2990;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2991;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2992;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2993;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta815<F: Float>(t1065: F, t15648: F, t15772: F, t3188: F, t1063: F, t16195: F, t3172: F, t16200: F, t15775: F, t16204: F, t16209: F, t10326: F, t1469: F, t1042: F, t11703: F, t16095: F, t16196: F, t3092: F, t3127: F, t43313: F, t4573: F, t4578: F, t4801: F, t906: F, t11922: F, t11927: F, t15621: F, t11671: F, t4954: F, t16068: F, t999: F, t11249: F, t4866: F, t12021: F, t4820: F, t11998: F, t15822: F, t11151: F, t11774: F, t15584: F, t15586: F, t15599: F, t15907: F, t15950: F, t16081: F, t16082: F, t16170: F, t1671: F, t3097: F, t3117: F, t3164: F, t42155: F, t42690: F, t42970: F, t4786: F, t4873: F, t15921: F, t3115: F, t1086: F, t15669: F, t3090: F, t43347: F, t53668: F, t16163: F, t3124: F, t11247: F, t11689: F, t11693: F, t11930: F, t15193: F, t15917: F, t16017: F, t16022: F, t16049: F, t16128: F, t19738: F, t19741: F, t3091: F, t42816: F, t42872: F, t53670: F, t54089: F, t11875: F, t15605: F, t11852: F, t41270: F, t15905: F, t43384: F, t15595: F, t43131: F, t11675: F, t15984: F, t11710: F, t15958: F, t11672: F, t15615: F, t15622: F, t15837: F, t15938: F, t15959: F, t16070: F, t43285: F, t53474: F, t11629: F, t53703: F, t3316: F, t4746: F, t4891: F, t16381: F, t11620: F, t11634: F, t11639: F, t11663: F, t11680: F, t11877: F, t15601: F, t15618: F, t15707: F, t15758: F, t15970: F, t16210: F, t357: F, t42571: F, t4825: F, t4893: F, t4899: F, t11262: F, t4874: F, t11631: F, t12116: F, t15906: F, t1592: F, t15968: F, t16048: F, t16147: F, t3154: F, t42550: F, t42833: F, t42883: F, t42886: F, t42889: F, t42892: F, t43069: F, t4583: F, t4892: F, t4896: F, t16055: F, t15833: F, t11779: F, t4845: F, t15749: F, t3211: F, t16148: F, t4837: F, t11656: F, t15791: F, t15908: F, t16052: F, t16138: F, t16144: F, t2858: F, t53459: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t54419, t54432, t54435, t54438, t54440, t54443, t54446, t54450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2985::<F>(t1065, t15648, t15772, t3188, t1063, t16195, t3172, t16200, t15775, t16204, t16209, t10326, t1469);
        let t54455 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2986::<F>(t1042, t1063, t11703, t16095, t16196, t3092, t3127, t3188, t43313, t4573, t4578, t4801, t54419, t54432, t54435, t54438, t54440, t54443, t54446, t54450, t906);
        let (t54469, t54471, t54474, t54479) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2987::<F>(t11922, t11927, t15621, t11671, t4954, t16068, t999, t11249, t4866);
        let t54495 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2988::<F>(t12021, t4820, t11998, t15822, t1042, t11151, t11774, t15584, t15586, t15599, t15907, t15950, t16081, t16082, t16170, t1671, t3097, t3117, t3127, t3164, t42155, t42690, t42970, t4786, t4873, t54469, t54471, t54474, t54479);
        let t54526 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2989::<F>(t11922, t15921, t3115, t1086, t15669, t3090, t43347, t53668, t16163, t3124, t11247, t11689, t11693, t11930, t15193, t15917, t16017, t16022, t16049, t16128, t19738, t19741, t3091, t3092, t3117, t42816, t42872, t4786, t53670, t54089);
        let (t54533, t54537, t54542, t54546, t54550) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2990::<F>(t11875, t11922, t15605, t11852, t41270, t15905, t43384, t15595, t3091, t43131, t11675, t15984);
        let t54559 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2991::<F>(t11710, t15958, t3091, t1042, t1063, t11672, t11675, t11927, t15615, t15622, t15837, t15938, t15959, t16070, t3117, t3188, t43285, t4786, t53474, t54533, t54537, t54542, t54546, t54550);
        let t54589 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2992::<F>(t11629, t53703, t3316, t4746, t4891, t16381, t3090, t11620, t11634, t11639, t11663, t11672, t11680, t11877, t15601, t15618, t15707, t15758, t15970, t16210, t19738, t3097, t3117, t3188, t357, t42571, t4825, t4893, t4899);
        let t54622 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2993::<F>(t11262, t3127, t4874, t11631, t12116, t15584, t15906, t1592, t15968, t16048, t16081, t16147, t3092, t3154, t42550, t42833, t42883, t42886, t42889, t42892, t43069, t4583, t4786, t4892, t4896);
        let t54653 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2994::<F>(t15758, t16055, t1063, t15833, t3172, t11779, t4845, t15749, t3211, t16148, t4837, t1042, t11656, t15791, t15906, t15908, t15970, t16052, t16138, t16144, t2858, t3117, t3127, t3188, t4801, t53459, t54479);
    (t54450, t54455, t54474, t54479, t54495, t54526, t54559, t54589, t54622, t54653)
}
