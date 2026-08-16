//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta954 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3173;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3174;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3175;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3176;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3177;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3178;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3179;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3180;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta954<F: Float>(t1794: F, t21082: F, t1250: F, t12832: F, t12866: F, t17351: F, t17459: F, t17649: F, t20771: F, t20938: F, t24753: F, t3629: F, t3718: F, t3720: F, t5332: F, t57223: F, t70476: F, t70491: F, t70493: F, t70496: F, t70630: F, t71238: F, t71300: F, t83033: F, t57242: F, t57251: F, t57257: F, t57271: F, t57274: F, t57331: F, t57548: F, t59330: F, t70511: F, t70521: F, t70523: F, t70542: F, t77513: F, t1256: F, t24681: F, t24671: F, t21233: F, t5391: F, t21271: F, t24846: F, t3647: F, t3670: F, t5386: F, t57550: F, t57606: F, t70581: F, t70583: F, t70612: F, t70616: F, t1261: F, t24240: F, t247: F, t3634: F, t21192: F, t5381: F, t1469: F, t17736: F, t17737: F, t17763: F, t20806: F, t20838: F, t21017: F, t21306: F, t24726: F, t3367: F, t3626: F, t4181: F, t5245: F, t5354: F, t6573: F, t6673: F, t6683: F, t70623: F, t71513: F, t12772: F, t24786: F, t3625: F, t1248: F, t13046: F, t13053: F, t17396: F, t21166: F, t24619: F, t24834: F, t44500: F, t44578: F, t44952: F, t45371: F, t45386: F, t471: F, t5407: F, t56947: F, t56953: F, t57422: F, t6429: F, t6690: F, t70794: F, t70995: F, t82838: F, t17572: F, t21188: F, t1042: F, t1214: F, t17235: F, t20792: F, t21272: F, t22671: F, t3711: F, t5270: F, t5279: F, t5296: F, t57136: F, t69795: F, t70664: F, t70667: F, t70672: F, t78785: F, t78790: F, t82543: F, t13052: F, t24667: F, t3172: F, t12916: F, t24705: F, t1222: F, t1791: F, t21095: F, t21177: F, t21275: F, t24741: F, t44624: F, t5308: F, t5320: F, t57464: F, t57471: F, t70469: F, t70685: F, t70689: F, t81194: F, t17240: F, t24244: F, t20982: F, t20986: F, t21126: F, t21129: F, t21239: F, t5312: F, t5373: F, t57480: F, t57491: F, t70733: F, t81173: F, t81182: F, t81212: F, t24648: F, t24633: F, t24228: F, t44895: F, t17569: F, t20864: F, t21184: F, t21267: F, t24644: F, t3719: F, t5302: F, t5384: F, t57229: F, t6635: F, t69968: F, t71585: F, t80045: F, t80050: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t83330, t83352) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3173::<F>(t1794, t21082, t1250, t12832, t12866, t17351, t17459, t17649, t20771, t20938, t24753, t3629, t3718, t3720, t5332, t57223, t70476, t70491, t70493, t70496, t70630, t71238, t71300, t83033);
        let t83361 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3174::<F>(t57242, t57251, t57257, t57271, t57274, t57331, t57548, t59330, t70511, t70521, t70523, t70542, t77513);
        let t83384 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3175::<F>(t1256, t24681, t24671, t21233, t5391, t21271, t24846, t3647, t3670, t5386, t57548, t57550, t57606, t70581, t70583, t70612, t70616, t77513);
        let t83414 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3176::<F>(t1261, t24240, t247, t3634, t21192, t5381, t1469, t17736, t17737, t17763, t20806, t20838, t21017, t21306, t24726, t3367, t3626, t3647, t4181, t5245, t5354, t6573, t6673, t6683, t70623, t71513);
        let t83451 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3177::<F>(t12772, t24786, t3625, t1248, t13046, t13053, t17396, t21166, t24619, t24834, t3626, t3720, t44500, t44578, t44952, t45371, t45386, t471, t5407, t56947, t56953, t57422, t6429, t6690, t70794, t70995, t82838);
        let t83480 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3178::<F>(t17572, t21188, t1042, t1214, t1261, t17235, t20792, t21272, t22671, t3711, t5270, t5279, t5296, t5391, t57136, t69795, t70664, t70667, t70672, t78785, t78790, t82543);
        let t83502 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3179::<F>(t13052, t24667, t3172, t12916, t24705, t3718, t1222, t1791, t21095, t21177, t21275, t24741, t44624, t5308, t5320, t57464, t57471, t70469, t70685, t70689, t81194);
        let t83526 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3180::<F>(t1222, t17240, t24244, t20982, t20986, t21126, t21129, t21239, t5312, t5373, t5391, t57480, t57491, t70733, t81173, t81182, t81212);
        let (t83551, t83562) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3181::<F>(t24648, t3172, t3711, t1214, t24633, t1261, t24228, t247, t44895, t1042, t17569, t20864, t21184, t21267, t24644, t3647, t3719, t5279, t5302, t5381, t5384, t57229, t6635, t69968, t71585, t80045, t80050);
    (t83330, t83352, t83361, t83384, t83414, t83451, t83480, t83502, t83526, t83551, t83562)
}
