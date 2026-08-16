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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta954(t1794: f64, t21082: f64, t1250: f64, t12832: f64, t12866: f64, t17351: f64, t17459: f64, t17649: f64, t20771: f64, t20938: f64, t24753: f64, t3629: f64, t3718: f64, t3720: f64, t5332: f64, t57223: f64, t70476: f64, t70491: f64, t70493: f64, t70496: f64, t70630: f64, t71238: f64, t71300: f64, t83033: f64, t57242: f64, t57251: f64, t57257: f64, t57271: f64, t57274: f64, t57331: f64, t57548: f64, t59330: f64, t70511: f64, t70521: f64, t70523: f64, t70542: f64, t77513: f64, t1256: f64, t24681: f64, t24671: f64, t21233: f64, t5391: f64, t21271: f64, t24846: f64, t3647: f64, t3670: f64, t5386: f64, t57550: f64, t57606: f64, t70581: f64, t70583: f64, t70612: f64, t70616: f64, t1261: f64, t24240: f64, t247: f64, t3634: f64, t21192: f64, t5381: f64, t1469: f64, t17736: f64, t17737: f64, t17763: f64, t20806: f64, t20838: f64, t21017: f64, t21306: f64, t24726: f64, t3367: f64, t3626: f64, t4181: f64, t5245: f64, t5354: f64, t6573: f64, t6673: f64, t6683: f64, t70623: f64, t71513: f64, t12772: f64, t24786: f64, t3625: f64, t1248: f64, t13046: f64, t13053: f64, t17396: f64, t21166: f64, t24619: f64, t24834: f64, t44500: f64, t44578: f64, t44952: f64, t45371: f64, t45386: f64, t471: f64, t5407: f64, t56947: f64, t56953: f64, t57422: f64, t6429: f64, t6690: f64, t70794: f64, t70995: f64, t82838: f64, t17572: f64, t21188: f64, t1042: f64, t1214: f64, t17235: f64, t20792: f64, t21272: f64, t22671: f64, t3711: f64, t5270: f64, t5279: f64, t5296: f64, t57136: f64, t69795: f64, t70664: f64, t70667: f64, t70672: f64, t78785: f64, t78790: f64, t82543: f64, t13052: f64, t24667: f64, t3172: f64, t12916: f64, t24705: f64, t1222: f64, t1791: f64, t21095: f64, t21177: f64, t21275: f64, t24741: f64, t44624: f64, t5308: f64, t5320: f64, t57464: f64, t57471: f64, t70469: f64, t70685: f64, t70689: f64, t81194: f64, t17240: f64, t24244: f64, t20982: f64, t20986: f64, t21126: f64, t21129: f64, t21239: f64, t5312: f64, t5373: f64, t57480: f64, t57491: f64, t70733: f64, t81173: f64, t81182: f64, t81212: f64, t24648: f64, t24633: f64, t24228: f64, t44895: f64, t17569: f64, t20864: f64, t21184: f64, t21267: f64, t24644: f64, t3719: f64, t5302: f64, t5384: f64, t57229: f64, t6635: f64, t69968: f64, t71585: f64, t80045: f64, t80050: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83330, t83352) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3173(t1794, t21082, t1250, t12832, t12866, t17351, t17459, t17649, t20771, t20938, t24753, t3629, t3718, t3720, t5332, t57223, t70476, t70491, t70493, t70496, t70630, t71238, t71300, t83033);
        let t83361 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3174(t57242, t57251, t57257, t57271, t57274, t57331, t57548, t59330, t70511, t70521, t70523, t70542, t77513);
        let t83384 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3175(t1256, t24681, t24671, t21233, t5391, t21271, t24846, t3647, t3670, t5386, t57548, t57550, t57606, t70581, t70583, t70612, t70616, t77513);
        let t83414 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3176(t1261, t24240, t247, t3634, t21192, t5381, t1469, t17736, t17737, t17763, t20806, t20838, t21017, t21306, t24726, t3367, t3626, t3647, t4181, t5245, t5354, t6573, t6673, t6683, t70623, t71513);
        let t83451 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3177(t12772, t24786, t3625, t1248, t13046, t13053, t17396, t21166, t24619, t24834, t3626, t3720, t44500, t44578, t44952, t45371, t45386, t471, t5407, t56947, t56953, t57422, t6429, t6690, t70794, t70995, t82838);
        let t83480 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3178(t17572, t21188, t1042, t1214, t1261, t17235, t20792, t21272, t22671, t3711, t5270, t5279, t5296, t5391, t57136, t69795, t70664, t70667, t70672, t78785, t78790, t82543);
        let t83502 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3179(t13052, t24667, t3172, t12916, t24705, t3718, t1222, t1791, t21095, t21177, t21275, t24741, t44624, t5308, t5320, t57464, t57471, t70469, t70685, t70689, t81194);
        let t83526 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3180(t1222, t17240, t24244, t20982, t20986, t21126, t21129, t21239, t5312, t5373, t5391, t57480, t57491, t70733, t81173, t81182, t81212);
        let (t83551, t83562) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3181(t24648, t3172, t3711, t1214, t24633, t1261, t24228, t247, t44895, t1042, t17569, t20864, t21184, t21267, t24644, t3647, t3719, t5279, t5302, t5381, t5384, t57229, t6635, t69968, t71585, t80045, t80050);
    (t83330, t83352, t83361, t83384, t83414, t83451, t83480, t83502, t83526, t83551, t83562)
}
