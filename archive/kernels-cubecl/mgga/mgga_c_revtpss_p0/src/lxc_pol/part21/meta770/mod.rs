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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta770<F: Float>(t50132: F, t50149: F, t10115: F, t1570: F, t11007: F, t1579: F, t252: F, t2771: F, t2782: F, t4322: F, t9292: F, t2772: F, t4321: F, t689: F, t11024: F, t1580: F, t10981: F, t22: F, t868: F, t15060: F, t2435: F, t14982: F, t2465: F, t2470: F, t10495: F, t14978: F, t14979: F, t2765: F, t2770: F, t39549: F, t39550: F, t39554: F, t41008: F, t4474: F, t865: F, t886: F, t2829: F, t15054: F, t786: F, t789: F, t4480: F, t9288: F, t1569: F, t2769: F, t10997: F, t10985: F, t15017: F, t39557: F, t39558: F, t39562: F, t39565: F, t39567: F, t39570: F, t39573: F, t40968: F, t40970: F, t40973: F, t40978: F, t15045: F, t15048: F, t2471: F, t15008: F, t10996: F, t14990: F, t41070: F, t14939: F, t212: F, t780: F, t2439: F, t4469: F, t785: F, t213: F, t2440: F, t4534: F, t41117: F, t10494: F, t15011: F, t40982: F, t40986: F, t40988: F, t40994: F, t40998: F, t10509: F, t10995: F, t122: F, t2466: F, t11008: F, t11009: F, t40999: F, t41003: F, t41004: F, t41006: F, t41014: F, t41018: F, t41021: F, t41026: F, t41029: F, t41032: F, t41034: F, t41037: F, t41078: F, t4533: F, t10777: F, t10779: F, t1548: F, t2646: F, t10868: F, t820: F, t844: F, t14896: F, t14701: F, t40731: F, t14468: F, t221: F, t2674: F, t2675: F, t14662: F, t231: F, t243: F, t2661: F, t2662: F, t14648: F, t14832: F, t2430: F, t14671: F, t14872: F, t10489: F, t10639: F, t14676: F, t2745: F, t2747: F, t40333: F, t40337: F, t40345: F, t40349: F, t40355: F, t40357: F, t40361: F, t40365: F, t4364: F, t4365: F, t4450: F, t10811: F, t14682: F, t14804: F, t14923: F, t4457: F, t837: F, t14853: F, t2652: F, t125: F, t14767: F, t14785: F, t2754: F, t40367: F, t40374: F, t40376: F, t40381: F, t40385: F, t40390: F, t40393: F, t40395: F, t40399: F, t40403: F) -> (F, F, F, F, F, F, F) {
        let (t50151, t50155, t50161, t50164, t50166, t50169) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2726::<F>(t50132, t50149, t10115, t1570, t11007, t1579, t252, t2771, t2782, t4322, t9292, t2772, t4321, t689);
        let (t50174, t50178, t50184, t50186) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2727::<F>(t11024, t1580, t689, t10981, t1579, t22, t868, t15060, t2435, t14982, t2465, t2470);
        let t50190 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2728::<F>(t50186, t10495, t14978, t14979, t1580, t2765, t2770, t39549, t39550, t39554, t41008, t4474, t50155, t50164, t50166, t50169, t50174, t50178, t50184, t865, t886);
        let (t50198, t50201, t50205, t50209) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2729::<F>(t2829, t4321, t689, t15054, t786, t789, t2465, t4480, t9288, t1569, t2769, t10997);
        let t50216 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2730::<F>(t10985, t15017, t39557, t39558, t39562, t39565, t39567, t39570, t39573, t40968, t40970, t40973, t40978, t50198, t50201, t50205, t50209);
        let (t50219, t50221, t50223, t50227, t50232) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2731::<F>(t15045, t2435, t15048, t2471, t15008, t10996, t14990, t41070, t14939, t212, t689, t780);
        let t50250 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2732::<F>(t2439, t4469, t780, t785, t213, t252, t2440, t4534, t1580, t41117, t10494, t15011, t2829, t40982, t40986, t40988, t40994, t40998, t50161, t50219, t50221, t50223, t50227, t50232);
        let t50276 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2733::<F>(t10509, t10995, t14990, t122, t14982, t2466, t11008, t11009, t1579, t2771, t40999, t41003, t41004, t41006, t41014, t41018, t41021, t41026, t41029, t41032, t41034, t41037, t41078, t4533, t865);
        let (t50292, t50296, t50299, t50303) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2734::<F>(t10777, t10779, t1548, t2646, t10868, t820, t844, t14896, t14701, t40731, t14468, t221, t2674, t2675);
        let (t50308, t50312, t50325) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2735::<F>(t14662, t231, t243, t2661, t2662, t14648, t14832, t2430, t10777, t10779, t14671, t14872);
        let t50327 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2736::<F>(t10489, t10639, t14676, t231, t2646, t2745, t2747, t40333, t40337, t40345, t40349, t40355, t40357, t40361, t40365, t4364, t4365, t4450, t50292, t50296, t50299, t50303, t50308, t50312, t50325);
        let t50365 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2737::<F>(t10811, t14682, t14804, t14923, t10777, t10779, t4457, t837, t14853, t2652, t125, t14468, t14676, t14767, t14785, t14872, t2745, t2747, t2754, t40367, t40374, t40376, t40381, t40385, t40390, t40393, t40395, t40399, t40403, t4364);
    (t50151, t50190, t50216, t50250, t50276, t50327, t50365)
}
