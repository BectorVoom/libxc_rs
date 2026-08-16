//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta835 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3128;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3129;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3130;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3131;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3132;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3133;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3134;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta835(t12898: f64, t1786: f64, t17202: f64, t372: f64, t15936: f64, t5405: f64, t17708: f64, t45769: f64, t44546: f64, t5340: f64, t5342: f64, t13041: f64, t56730: f64, t11772: f64, t17394: f64, t3717: f64, t12865: f64, t17400: f64, t1042: f64, t12629: f64, t12866: f64, t12868: f64, t12945: f64, t12956: f64, t13048: f64, t1469: f64, t17344: f64, t17351: f64, t17536: f64, t17539: f64, t17649: f64, t17651: f64, t17672: f64, t17693: f64, t17713: f64, t17799: f64, t247: f64, t3719: f64, t44230: f64, t44561: f64, t44607: f64, t44616: f64, t5296: f64, t5297: f64, t5384: f64, t5391: f64, t5407: f64, t56543: f64, t56907: f64, t1222: f64, t1781: f64, t2438: f64, t12886: f64, t3601: f64, t5245: f64, t12854: f64, t21013: f64, t12808: f64, t3698: f64, t5047: f64, t697: f64, t1248: f64, t12784: f64, t12805: f64, t12809: f64, t12812: f64, t12855: f64, t12858: f64, t12872: f64, t12910: f64, t13076: f64, t16775: f64, t1715: f64, t17396: f64, t17500: f64, t17514: f64, t17674: f64, t17677: f64, t17682: f64, t21014: f64, t3372: f64, t3604: f64, t3611: f64, t3625: f64, t3626: f64, t3711: f64, t3720: f64, t44431: f64, t44521: f64, t44634: f64, t44637: f64, t471: f64, t5056: f64, t5274: f64, t5277: f64, t5331: f64, t12916: f64, t17455: f64, t3584: f64, t5333: f64, t16738: f64, t17240: f64, t16742: f64, t16733: f64, t13036: f64, t13039: f64, t57403: f64, t3597: f64, t12772: f64, t17678: f64, t17683: f64, t12876: f64, t13055: f64, t16771: f64, t17461: f64, t21306: f64, t44624: f64, t44649: f64, t44658: f64, t44661: f64, t44672: f64, t5308: f64, t5332: f64, t56153: f64, t56224: f64, t12832: f64, t17620: f64, t17412: f64, t3636: f64, t1196: f64, t12500: f64, t16672: f64, t12227: f64, t1732: f64, t1149: f64, t12230: f64, t3427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57615, t57621, t57622, t57631, t57636, t57641) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3128(t12898, t1786, t17202, t372, t15936, t5405, t17708, t45769, t44546, t5340, t5342, t13041, t56730);
        let t57667 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3129(t11772, t17394, t3717, t12865, t17400, t1042, t12629, t12866, t12868, t12945, t12956, t13048, t1469, t17344, t17351, t17536, t17539, t17649, t17651, t17672, t17693, t17713, t17799, t247, t3719, t44230, t44561, t44607, t44616, t5296, t5297, t5384, t5391, t5405, t5407, t56543, t56907, t57615, t57621, t57622, t57631, t57636, t57641);
        let (t57687, t57689, t57696, t57707, t57710, t57726) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3130(t1222, t1781, t2438, t12886, t5391, t3601, t5245, t12854, t21013, t12808, t3698, t5047, t697);
        let t57728 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3131(t57726, t1042, t1248, t12784, t12805, t12809, t12812, t12855, t12858, t12872, t12910, t13076, t16775, t1715, t17396, t17500, t17514, t17674, t17677, t17682, t21014, t3372, t3604, t3611, t3625, t3626, t3711, t372, t3720, t44431, t44521, t44634, t44637, t471, t5056, t5274, t5277, t5331, t5340, t5405, t57687, t57689, t57696, t57707, t57710);
        let (t57735, t57737, t57743, t57746, t57749, t57759) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3132(t12855, t12916, t17455, t3584, t5333, t1222, t16738, t17240, t16742, t16733, t13036, t13039, t57403);
        let t57779 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3133(t13036, t3597, t57403, t12772, t17678, t5340, t17683, t5331, t1222, t12809, t12876, t12910, t13048, t13055, t16771, t17461, t21306, t3720, t44624, t44649, t44658, t44661, t44672, t5308, t5332, t5405, t56153, t56224, t57735, t57737, t57743, t57746, t57749, t57759);
        let (t57780, t57786, t57794, t57799) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3134(t12832, t17620, t17412, t3636, t1196, t12500, t16672, t12227, t1732, t1149, t12230, t3427);
    (t57622, t57667, t57696, t57728, t57737, t57779, t57780, t57786, t57794, t57799)
}
