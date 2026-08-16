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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta700<F: Float>(t849: F, t98832: F, t23083: F, t28375: F, t28396: F, t81835: F, t58853: F, t6605: F, t828: F, t9972: F, t4250: F, t87199: F, t81912: F, t87412: F, t87426: F, t92676: F, t92677: F, t92689: F, t98818: F, t98820: F, t98822: F, t98824: F, t98826: F, t98828: F, t98830: F, t16918: F, t23146: F, t16898: F, t4191: F, t4240: F, t232: F, t58569: F, t815: F, t2628: F, t5585: F, t16949: F, t221: F, t25154: F, t25119: F, t841: F, t81921: F, t81928: F, t81934: F, t81943: F, t81955: F, t87444: F, t87445: F, t87464: F, t87478: F, t87488: F, t98644: F, t98688: F, t98713: F, t98740: F, t98795: F, t98816: F, t25038: F, t25248: F, t776: F, t98422: F, t23110: F, t23185: F, t28321: F, t16805: F, t1909: F, t226: F, t235: F, t25256: F, t28407: F, t4166: F, t4291: F, t808: F, t812: F, t82032: F, t82039: F, t82047: F, t829: F, t87710: F, t87714: F, t87730: F, t87734: F, t92817: F, t98524: F, t98592: F, t98601: F, t98608: F, t13065: F, t1492: F, t1527: F, t1912: F, t23281: F, t25160: F, t25188: F, t25329: F, t259: F, t2597: F, t2713: F, t2718: F, t28406: F, t28432: F, t4301: F, t5658: F, t58143: F, t59466: F, t59519: F, t7538: F, t798: F, t82147: F, t82154: F, t855: F, t858: F, t87029: F, t87050: F, t87754: F, t98315: F, t98319: F, t98322: F, t98370: F, t98409: F, t98450: F, t98497: F, t98536: F, t98566: F, t98587: F, t23168: F, t28277: F, t28295: F, t6547: F, t6562: F, t7488: F, t86893: F, t28439: F, t28268: F, t81591: F, t17049: F, t1880: F, t6553: F, t6571: F, t17092: F, t25200: F, t4147: F, t4300: F, t6663: F, t7537: F, t82209: F, t82211: F, t82219: F, t87805: F, t23270: F, t25191: F, t23204: F, t28294: F, t1493: F, t254: F, t28263: F, t23237: F, t28299: F, t81979: F, t28273: F, t13042: F, t17052: F, t17090: F, t218: F, t25170: F, t25330: F, t6632: F, t7517: F, t82259: F, t28264: F, t225: F, t28282: F, t17022: F, t214: F, t258: F, t28272: F, t794: F, t25224: F, t25341: F, t6552: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98833, t98836, t98838, t98842, t98844) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2253::<F>(t849, t98832, t23083, t28375, t28396, t81835, t58853, t6605, t828, t9972, t4250, t87199);
        let t98846 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2254::<F>(t81912, t87412, t87426, t92676, t92677, t92689, t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98833, t98836, t98838, t98842, t98844);
        let (t98847, t98849, t98851, t98853, t98858, t98862) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2255::<F>(t16918, t23146, t16898, t4191, t87199, t4240, t232, t58569, t6605, t815, t2628, t5585, t828);
        let t98873 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2256::<F>(t16949, t221, t25154, t25119, t841, t81921, t81928, t81934, t81943, t81955, t87444, t87445, t87464, t87478, t87488, t98847, t98849, t98851, t98853, t98858, t98862);
        let (t98876, t98881) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2257::<F>(t98644, t98688, t98713, t98740, t98795, t98816, t98846, t98873, t25038, t25248, t776, t98422);
        let t98886 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2258::<F>(t23110, t23185, t28321, t16805, t1909, t226, t235, t25256, t28407, t4166, t4291, t808, t812, t82032, t82039, t82047, t829, t87710, t87714, t87730, t87734, t92817, t98524, t98592, t98601, t98608, t98876, t98881);
        let t98913 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2259::<F>(t13065, t1492, t1527, t1912, t23281, t25160, t25188, t25329, t259, t2597, t2713, t2718, t28406, t28432, t4301, t5658, t58143, t59466, t59519, t7538, t798, t82147, t82154, t855, t858, t87029, t87050, t87754, t98315, t98319, t98322, t98370, t98409, t98450, t98497, t98536, t98566, t98587, t98886);
        let (t98921, t98923, t98927, t98932, t98941, t98945) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2260::<F>(t23168, t28277, t28295, t6547, t6562, t7488, t86893, t28439, t28268, t81591, t17049, t1880, t6553, t6571);
        let t98947 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2261::<F>(t17092, t25200, t2718, t4147, t4300, t6663, t7537, t82209, t82211, t82219, t855, t87805, t98927, t98932, t98941, t98945);
        let (t98963, t98966, t98975, t98983, t98986) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2262::<F>(t1527, t776, t23270, t25038, t25191, t23204, t28294, t6562, t1493, t254, t28263, t1880, t23237);
        let t98999 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2263::<F>(t28299, t81979, t28273, t6547, t13042, t17052, t17090, t218, t25170, t25330, t259, t4147, t6632, t7517, t82259, t98876, t98975, t98983, t98986);
        let (t99003, t99010, t99019, t99022, t99033) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2264::<F>(t28264, t6547, t225, t28282, t17022, t1880, t214, t258, t28272, t6562, t794, t25224, t25341, t6552);
    (t98913, t98921, t98923, t98947, t98963, t98966, t98999, t99003, t99010, t99019, t99022, t99033)
}
