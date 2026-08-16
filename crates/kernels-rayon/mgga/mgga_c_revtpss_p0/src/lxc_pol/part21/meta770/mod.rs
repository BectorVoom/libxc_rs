//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta770 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2726;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2727;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2728;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2729;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2730;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2731;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2732;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2733;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2734;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2735;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2736;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta770(t50132: f64, t50149: f64, t10115: f64, t1570: f64, t11007: f64, t1579: f64, t252: f64, t2771: f64, t2782: f64, t4322: f64, t9292: f64, t2772: f64, t4321: f64, t689: f64, t11024: f64, t1580: f64, t10981: f64, t22: f64, t868: f64, t15060: f64, t2435: f64, t14982: f64, t2465: f64, t2470: f64, t10495: f64, t14978: f64, t14979: f64, t2765: f64, t2770: f64, t39549: f64, t39550: f64, t39554: f64, t41008: f64, t4474: f64, t865: f64, t886: f64, t2829: f64, t15054: f64, t786: f64, t789: f64, t4480: f64, t9288: f64, t1569: f64, t2769: f64, t10997: f64, t10985: f64, t15017: f64, t39557: f64, t39558: f64, t39562: f64, t39565: f64, t39567: f64, t39570: f64, t39573: f64, t40968: f64, t40970: f64, t40973: f64, t40978: f64, t15045: f64, t15048: f64, t2471: f64, t15008: f64, t10996: f64, t14990: f64, t41070: f64, t14939: f64, t212: f64, t780: f64, t2439: f64, t4469: f64, t785: f64, t213: f64, t2440: f64, t4534: f64, t41117: f64, t10494: f64, t15011: f64, t40982: f64, t40986: f64, t40988: f64, t40994: f64, t40998: f64, t10509: f64, t10995: f64, t122: f64, t2466: f64, t11008: f64, t11009: f64, t40999: f64, t41003: f64, t41004: f64, t41006: f64, t41014: f64, t41018: f64, t41021: f64, t41026: f64, t41029: f64, t41032: f64, t41034: f64, t41037: f64, t41078: f64, t4533: f64, t10777: f64, t10779: f64, t1548: f64, t2646: f64, t10868: f64, t820: f64, t844: f64, t14896: f64, t14701: f64, t40731: f64, t14468: f64, t221: f64, t2674: f64, t2675: f64, t14662: f64, t231: f64, t243: f64, t2661: f64, t2662: f64, t14648: f64, t14832: f64, t2430: f64, t14671: f64, t14872: f64, t10489: f64, t10639: f64, t14676: f64, t2745: f64, t2747: f64, t40333: f64, t40337: f64, t40345: f64, t40349: f64, t40355: f64, t40357: f64, t40361: f64, t40365: f64, t4364: f64, t4365: f64, t4450: f64, t10811: f64, t14682: f64, t14804: f64, t14923: f64, t4457: f64, t837: f64, t14853: f64, t2652: f64, t125: f64, t14767: f64, t14785: f64, t2754: f64, t40367: f64, t40374: f64, t40376: f64, t40381: f64, t40385: f64, t40390: f64, t40393: f64, t40395: f64, t40399: f64, t40403: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t50151, t50155, t50161, t50164, t50166, t50169) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2726(t50132, t50149, t10115, t1570, t11007, t1579, t252, t2771, t2782, t4322, t9292, t2772, t4321, t689);
        let (t50174, t50178, t50184, t50186) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2727(t11024, t1580, t689, t10981, t1579, t22, t868, t15060, t2435, t14982, t2465, t2470);
        let t50190 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2728(t50186, t10495, t14978, t14979, t1580, t2765, t2770, t39549, t39550, t39554, t41008, t4474, t50155, t50164, t50166, t50169, t50174, t50178, t50184, t865, t886);
        let (t50198, t50201, t50205, t50209) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2729(t2829, t4321, t689, t15054, t786, t789, t2465, t4480, t9288, t1569, t2769, t10997);
        let t50216 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2730(t10985, t15017, t39557, t39558, t39562, t39565, t39567, t39570, t39573, t40968, t40970, t40973, t40978, t50198, t50201, t50205, t50209);
        let (t50219, t50221, t50223, t50227, t50232) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2731(t15045, t2435, t15048, t2471, t15008, t10996, t14990, t41070, t14939, t212, t689, t780);
        let t50250 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2732(t2439, t4469, t780, t785, t213, t252, t2440, t4534, t1580, t41117, t10494, t15011, t2829, t40982, t40986, t40988, t40994, t40998, t50161, t50219, t50221, t50223, t50227, t50232);
        let t50276 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2733(t10509, t10995, t14990, t122, t14982, t2466, t11008, t11009, t1579, t2771, t40999, t41003, t41004, t41006, t41014, t41018, t41021, t41026, t41029, t41032, t41034, t41037, t41078, t4533, t865);
        let (t50292, t50296, t50299, t50303) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2734(t10777, t10779, t1548, t2646, t10868, t820, t844, t14896, t14701, t40731, t14468, t221, t2674, t2675);
        let (t50308, t50312, t50325) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2735(t14662, t231, t243, t2661, t2662, t14648, t14832, t2430, t10777, t10779, t14671, t14872);
        let t50327 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2736(t10489, t10639, t14676, t231, t2646, t2745, t2747, t40333, t40337, t40345, t40349, t40355, t40357, t40361, t40365, t4364, t4365, t4450, t50292, t50296, t50299, t50303, t50308, t50312, t50325);
        let t50365 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2737(t10811, t14682, t14804, t14923, t10777, t10779, t4457, t837, t14853, t2652, t125, t14468, t14676, t14767, t14785, t14872, t2745, t2747, t2754, t40367, t40374, t40376, t40381, t40385, t40390, t40393, t40395, t40399, t40403, t4364);
    (t50151, t50190, t50216, t50250, t50276, t50327, t50365)
}
