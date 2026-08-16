//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta700 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2253;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2254;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2255;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2256;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2257;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2258;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2259;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2260;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2261;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2262;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2263;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta700(t849: f64, t98832: f64, t23083: f64, t28375: f64, t28396: f64, t81835: f64, t58853: f64, t6605: f64, t828: f64, t9972: f64, t4250: f64, t87199: f64, t81912: f64, t87412: f64, t87426: f64, t92676: f64, t92677: f64, t92689: f64, t98818: f64, t98820: f64, t98822: f64, t98824: f64, t98826: f64, t98828: f64, t98830: f64, t16918: f64, t23146: f64, t16898: f64, t4191: f64, t4240: f64, t232: f64, t58569: f64, t815: f64, t2628: f64, t5585: f64, t16949: f64, t221: f64, t25154: f64, t25119: f64, t841: f64, t81921: f64, t81928: f64, t81934: f64, t81943: f64, t81955: f64, t87444: f64, t87445: f64, t87464: f64, t87478: f64, t87488: f64, t98644: f64, t98688: f64, t98713: f64, t98740: f64, t98795: f64, t98816: f64, t25038: f64, t25248: f64, t776: f64, t98422: f64, t23110: f64, t23185: f64, t28321: f64, t16805: f64, t1909: f64, t226: f64, t235: f64, t25256: f64, t28407: f64, t4166: f64, t4291: f64, t808: f64, t812: f64, t82032: f64, t82039: f64, t82047: f64, t829: f64, t87710: f64, t87714: f64, t87730: f64, t87734: f64, t92817: f64, t98524: f64, t98592: f64, t98601: f64, t98608: f64, t13065: f64, t1492: f64, t1527: f64, t1912: f64, t23281: f64, t25160: f64, t25188: f64, t25329: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t28406: f64, t28432: f64, t4301: f64, t5658: f64, t58143: f64, t59466: f64, t59519: f64, t7538: f64, t798: f64, t82147: f64, t82154: f64, t855: f64, t858: f64, t87029: f64, t87050: f64, t87754: f64, t98315: f64, t98319: f64, t98322: f64, t98370: f64, t98409: f64, t98450: f64, t98497: f64, t98536: f64, t98566: f64, t98587: f64, t23168: f64, t28277: f64, t28295: f64, t6547: f64, t6562: f64, t7488: f64, t86893: f64, t28439: f64, t28268: f64, t81591: f64, t17049: f64, t1880: f64, t6553: f64, t6571: f64, t17092: f64, t25200: f64, t4147: f64, t4300: f64, t6663: f64, t7537: f64, t82209: f64, t82211: f64, t82219: f64, t87805: f64, t23270: f64, t25191: f64, t23204: f64, t28294: f64, t1493: f64, t254: f64, t28263: f64, t23237: f64, t28299: f64, t81979: f64, t28273: f64, t13042: f64, t17052: f64, t17090: f64, t218: f64, t25170: f64, t25330: f64, t6632: f64, t7517: f64, t82259: f64, t28264: f64, t225: f64, t28282: f64, t17022: f64, t214: f64, t258: f64, t28272: f64, t794: f64, t25224: f64, t25341: f64, t6552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98833, t98836, t98838, t98842, t98844) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2253(t849, t98832, t23083, t28375, t28396, t81835, t58853, t6605, t828, t9972, t4250, t87199);
        let t98846 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2254(t81912, t87412, t87426, t92676, t92677, t92689, t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98833, t98836, t98838, t98842, t98844);
        let (t98847, t98849, t98851, t98853, t98858, t98862) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2255(t16918, t23146, t16898, t4191, t87199, t4240, t232, t58569, t6605, t815, t2628, t5585, t828);
        let t98873 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2256(t16949, t221, t25154, t25119, t841, t81921, t81928, t81934, t81943, t81955, t87444, t87445, t87464, t87478, t87488, t98847, t98849, t98851, t98853, t98858, t98862);
        let (t98876, t98881) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2257(t98644, t98688, t98713, t98740, t98795, t98816, t98846, t98873, t25038, t25248, t776, t98422);
        let t98886 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2258(t23110, t23185, t28321, t16805, t1909, t226, t235, t25256, t28407, t4166, t4291, t808, t812, t82032, t82039, t82047, t829, t87710, t87714, t87730, t87734, t92817, t98524, t98592, t98601, t98608, t98876, t98881);
        let t98913 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2259(t13065, t1492, t1527, t1912, t23281, t25160, t25188, t25329, t259, t2597, t2713, t2718, t28406, t28432, t4301, t5658, t58143, t59466, t59519, t7538, t798, t82147, t82154, t855, t858, t87029, t87050, t87754, t98315, t98319, t98322, t98370, t98409, t98450, t98497, t98536, t98566, t98587, t98886);
        let (t98921, t98923, t98927, t98932, t98941, t98945) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2260(t23168, t28277, t28295, t6547, t6562, t7488, t86893, t28439, t28268, t81591, t17049, t1880, t6553, t6571);
        let t98947 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2261(t17092, t25200, t2718, t4147, t4300, t6663, t7537, t82209, t82211, t82219, t855, t87805, t98927, t98932, t98941, t98945);
        let (t98963, t98966, t98975, t98983, t98986) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2262(t1527, t776, t23270, t25038, t25191, t23204, t28294, t6562, t1493, t254, t28263, t1880, t23237);
        let t98999 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2263(t28299, t81979, t28273, t6547, t13042, t17052, t17090, t218, t25170, t25330, t259, t4147, t6632, t7517, t82259, t98876, t98975, t98983, t98986);
        let (t99003, t99010, t99019, t99022, t99033) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2264(t28264, t6547, t225, t28282, t17022, t1880, t214, t258, t28272, t6562, t794, t25224, t25341, t6552);
    (t98913, t98921, t98923, t98947, t98963, t98966, t98999, t99003, t99010, t99019, t99022, t99033)
}
