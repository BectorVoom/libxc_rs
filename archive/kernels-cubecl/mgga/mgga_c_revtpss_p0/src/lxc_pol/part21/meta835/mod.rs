//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta835 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3128;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3129;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3130;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3131;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3132;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3133;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3134;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta835<F: Float>(t12898: F, t1786: F, t17202: F, t372: F, t15936: F, t5405: F, t17708: F, t45769: F, t44546: F, t5340: F, t5342: F, t13041: F, t56730: F, t11772: F, t17394: F, t3717: F, t12865: F, t17400: F, t1042: F, t12629: F, t12866: F, t12868: F, t12945: F, t12956: F, t13048: F, t1469: F, t17344: F, t17351: F, t17536: F, t17539: F, t17649: F, t17651: F, t17672: F, t17693: F, t17713: F, t17799: F, t247: F, t3719: F, t44230: F, t44561: F, t44607: F, t44616: F, t5296: F, t5297: F, t5384: F, t5391: F, t5407: F, t56543: F, t56907: F, t1222: F, t1781: F, t2438: F, t12886: F, t3601: F, t5245: F, t12854: F, t21013: F, t12808: F, t3698: F, t5047: F, t697: F, t1248: F, t12784: F, t12805: F, t12809: F, t12812: F, t12855: F, t12858: F, t12872: F, t12910: F, t13076: F, t16775: F, t1715: F, t17396: F, t17500: F, t17514: F, t17674: F, t17677: F, t17682: F, t21014: F, t3372: F, t3604: F, t3611: F, t3625: F, t3626: F, t3711: F, t3720: F, t44431: F, t44521: F, t44634: F, t44637: F, t471: F, t5056: F, t5274: F, t5277: F, t5331: F, t12916: F, t17455: F, t3584: F, t5333: F, t16738: F, t17240: F, t16742: F, t16733: F, t13036: F, t13039: F, t57403: F, t3597: F, t12772: F, t17678: F, t17683: F, t12876: F, t13055: F, t16771: F, t17461: F, t21306: F, t44624: F, t44649: F, t44658: F, t44661: F, t44672: F, t5308: F, t5332: F, t56153: F, t56224: F, t12832: F, t17620: F, t17412: F, t3636: F, t1196: F, t12500: F, t16672: F, t12227: F, t1732: F, t1149: F, t12230: F, t3427: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t57615, t57621, t57622, t57631, t57636, t57641) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3128::<F>(t12898, t1786, t17202, t372, t15936, t5405, t17708, t45769, t44546, t5340, t5342, t13041, t56730);
        let t57667 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3129::<F>(t11772, t17394, t3717, t12865, t17400, t1042, t12629, t12866, t12868, t12945, t12956, t13048, t1469, t17344, t17351, t17536, t17539, t17649, t17651, t17672, t17693, t17713, t17799, t247, t3719, t44230, t44561, t44607, t44616, t5296, t5297, t5384, t5391, t5405, t5407, t56543, t56907, t57615, t57621, t57622, t57631, t57636, t57641);
        let (t57687, t57689, t57696, t57707, t57710, t57726) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3130::<F>(t1222, t1781, t2438, t12886, t5391, t3601, t5245, t12854, t21013, t12808, t3698, t5047, t697);
        let t57728 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3131::<F>(t57726, t1042, t1248, t12784, t12805, t12809, t12812, t12855, t12858, t12872, t12910, t13076, t16775, t1715, t17396, t17500, t17514, t17674, t17677, t17682, t21014, t3372, t3604, t3611, t3625, t3626, t3711, t372, t3720, t44431, t44521, t44634, t44637, t471, t5056, t5274, t5277, t5331, t5340, t5405, t57687, t57689, t57696, t57707, t57710);
        let (t57735, t57737, t57743, t57746, t57749, t57759) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3132::<F>(t12855, t12916, t17455, t3584, t5333, t1222, t16738, t17240, t16742, t16733, t13036, t13039, t57403);
        let t57779 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3133::<F>(t13036, t3597, t57403, t12772, t17678, t5340, t17683, t5331, t1222, t12809, t12876, t12910, t13048, t13055, t16771, t17461, t21306, t3720, t44624, t44649, t44658, t44661, t44672, t5308, t5332, t5405, t56153, t56224, t57735, t57737, t57743, t57746, t57749, t57759);
        let (t57780, t57786, t57794, t57799) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3134::<F>(t12832, t17620, t17412, t3636, t1196, t12500, t16672, t12227, t1732, t1149, t12230, t3427);
    (t57622, t57667, t57696, t57728, t57737, t57779, t57780, t57786, t57794, t57799)
}
