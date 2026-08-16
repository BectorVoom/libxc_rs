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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta815(t1065: f64, t15648: f64, t15772: f64, t3188: f64, t1063: f64, t16195: f64, t3172: f64, t16200: f64, t15775: f64, t16204: f64, t16209: f64, t10326: f64, t1469: f64, t1042: f64, t11703: f64, t16095: f64, t16196: f64, t3092: f64, t3127: f64, t43313: f64, t4573: f64, t4578: f64, t4801: f64, t906: f64, t11922: f64, t11927: f64, t15621: f64, t11671: f64, t4954: f64, t16068: f64, t999: f64, t11249: f64, t4866: f64, t12021: f64, t4820: f64, t11998: f64, t15822: f64, t11151: f64, t11774: f64, t15584: f64, t15586: f64, t15599: f64, t15907: f64, t15950: f64, t16081: f64, t16082: f64, t16170: f64, t1671: f64, t3097: f64, t3117: f64, t3164: f64, t42155: f64, t42690: f64, t42970: f64, t4786: f64, t4873: f64, t15921: f64, t3115: f64, t1086: f64, t15669: f64, t3090: f64, t43347: f64, t53668: f64, t16163: f64, t3124: f64, t11247: f64, t11689: f64, t11693: f64, t11930: f64, t15193: f64, t15917: f64, t16017: f64, t16022: f64, t16049: f64, t16128: f64, t19738: f64, t19741: f64, t3091: f64, t42816: f64, t42872: f64, t53670: f64, t54089: f64, t11875: f64, t15605: f64, t11852: f64, t41270: f64, t15905: f64, t43384: f64, t15595: f64, t43131: f64, t11675: f64, t15984: f64, t11710: f64, t15958: f64, t11672: f64, t15615: f64, t15622: f64, t15837: f64, t15938: f64, t15959: f64, t16070: f64, t43285: f64, t53474: f64, t11629: f64, t53703: f64, t3316: f64, t4746: f64, t4891: f64, t16381: f64, t11620: f64, t11634: f64, t11639: f64, t11663: f64, t11680: f64, t11877: f64, t15601: f64, t15618: f64, t15707: f64, t15758: f64, t15970: f64, t16210: f64, t357: f64, t42571: f64, t4825: f64, t4893: f64, t4899: f64, t11262: f64, t4874: f64, t11631: f64, t12116: f64, t15906: f64, t1592: f64, t15968: f64, t16048: f64, t16147: f64, t3154: f64, t42550: f64, t42833: f64, t42883: f64, t42886: f64, t42889: f64, t42892: f64, t43069: f64, t4583: f64, t4892: f64, t4896: f64, t16055: f64, t15833: f64, t11779: f64, t4845: f64, t15749: f64, t3211: f64, t16148: f64, t4837: f64, t11656: f64, t15791: f64, t15908: f64, t16052: f64, t16138: f64, t16144: f64, t2858: f64, t53459: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54419, t54432, t54435, t54438, t54440, t54443, t54446, t54450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2985(t1065, t15648, t15772, t3188, t1063, t16195, t3172, t16200, t15775, t16204, t16209, t10326, t1469);
        let t54455 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2986(t1042, t1063, t11703, t16095, t16196, t3092, t3127, t3188, t43313, t4573, t4578, t4801, t54419, t54432, t54435, t54438, t54440, t54443, t54446, t54450, t906);
        let (t54469, t54471, t54474, t54479) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2987(t11922, t11927, t15621, t11671, t4954, t16068, t999, t11249, t4866);
        let t54495 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2988(t12021, t4820, t11998, t15822, t1042, t11151, t11774, t15584, t15586, t15599, t15907, t15950, t16081, t16082, t16170, t1671, t3097, t3117, t3127, t3164, t42155, t42690, t42970, t4786, t4873, t54469, t54471, t54474, t54479);
        let t54526 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2989(t11922, t15921, t3115, t1086, t15669, t3090, t43347, t53668, t16163, t3124, t11247, t11689, t11693, t11930, t15193, t15917, t16017, t16022, t16049, t16128, t19738, t19741, t3091, t3092, t3117, t42816, t42872, t4786, t53670, t54089);
        let (t54533, t54537, t54542, t54546, t54550) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2990(t11875, t11922, t15605, t11852, t41270, t15905, t43384, t15595, t3091, t43131, t11675, t15984);
        let t54559 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2991(t11710, t15958, t3091, t1042, t1063, t11672, t11675, t11927, t15615, t15622, t15837, t15938, t15959, t16070, t3117, t3188, t43285, t4786, t53474, t54533, t54537, t54542, t54546, t54550);
        let t54589 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2992(t11629, t53703, t3316, t4746, t4891, t16381, t3090, t11620, t11634, t11639, t11663, t11672, t11680, t11877, t15601, t15618, t15707, t15758, t15970, t16210, t19738, t3097, t3117, t3188, t357, t42571, t4825, t4893, t4899);
        let t54622 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2993(t11262, t3127, t4874, t11631, t12116, t15584, t15906, t1592, t15968, t16048, t16081, t16147, t3092, t3154, t42550, t42833, t42883, t42886, t42889, t42892, t43069, t4583, t4786, t4892, t4896);
        let t54653 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2994(t15758, t16055, t1063, t15833, t3172, t11779, t4845, t15749, t3211, t16148, t4837, t1042, t11656, t15791, t15906, t15908, t15970, t16052, t16138, t16144, t2858, t3117, t3127, t3188, t4801, t53459, t54479);
    (t54450, t54455, t54474, t54479, t54495, t54526, t54559, t54589, t54622, t54653)
}
