//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1785;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1786;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1787;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1788;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1789;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1790;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1791;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta579(t3628: f64, t5825: f64, t6573: f64, t1235: f64, t17709: f64, t1791: f64, t20851: f64, t20956: f64, t21063: f64, t24636: f64, t371: f64, t372: f64, t3720: f64, t44844: f64, t482: f64, t5327: f64, t6611: f64, t6647: f64, t70263: f64, t70278: f64, t70578: f64, t83109: f64, t84098: f64, t84636: f64, t89960: f64, t1012: f64, t1222: f64, t1225: f64, t1782: f64, t21213: f64, t21306: f64, t24736: f64, t24821: f64, t24827: f64, t24831: f64, t24836: f64, t3699: f64, t44348: f64, t44919: f64, t5373: f64, t57707: f64, t6653: f64, t83962: f64, t87107: f64, t87126: f64, t87145: f64, t1261: f64, t12855: f64, t13100: f64, t17475: f64, t21040: f64, t24228: f64, t24535: f64, t247: f64, t24792: f64, t3604: f64, t3625: f64, t3626: f64, t44225: f64, t5312: f64, t5381: f64, t83392: f64, t83394: f64, t83435: f64, t89822: f64, t89826: f64, t89863: f64, t90042: f64, t90262: f64, t6587: f64, t6622: f64, t1042: f64, t12787: f64, t1715: f64, t20795: f64, t20809: f64, t3711: f64, t44190: f64, t5340: f64, t57471: f64, t5819: f64, t6429: f64, t6640: f64, t6690: f64, t70758: f64, t71275: f64, t71513: f64, t82816: f64, t83504: f64, t83539: f64, t83558: f64, t83580: f64, t58777: f64, t70942: f64, t83699: f64, t83719: f64, t83731: f64, t83735: f64, t83748: f64, t83751: f64, t83758: f64, t83783: f64, t83798: f64, t24677: f64, t467: f64, t475: f64, t484: f64, t52: f64, t6594: f64, t6601: f64, t71187: f64, t71192: f64, t83849: f64, t83851: f64, t83860: f64, t83863: f64, t83871: f64, t83891: f64, t83897: f64, rho1: f64, t1774: f64, t471: f64, t12866: f64, t17344: f64, t17694: f64, t1797: f64, t20820: f64, t24652: f64, t24655: f64, t24808: f64, t3718: f64, t5268: f64, t6625: f64, t82725: f64, t82799: f64, t83607: f64, t83992: f64, t83994: f64, t88916: f64, t90885: f64, t17693: f64, t17729: f64, t17747: f64, t1785: f64, t225: f64, t24647: f64, t24680: f64, t480: f64, t5046: f64, t59144: f64, t71718: f64, t71744: f64, t84029: f64, t84032: f64, t84061: f64, t84645: f64, t89883: f64, t90881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91012, t91037, t91060) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1785(t3628, t5825, t6573, t1235, t17709, t1791, t20851, t20956, t21063, t24636, t371, t372, t3720, t44844, t482, t5327, t6611, t6647, t70263, t70278, t70578, t83109, t84098, t84636, t89960);
        let t91119 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1786(t1012, t1222, t1225, t1782, t21213, t21306, t24736, t24821, t24827, t24831, t24836, t3699, t44348, t44919, t5373, t57707, t6653, t83962, t87107, t87126, t87145);
        let t91173 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1787(t1222, t1261, t12855, t13100, t17475, t21040, t24228, t24535, t247, t24792, t3604, t3625, t3626, t3720, t44225, t5312, t5381, t83392, t83394, t83435, t89822, t89826, t89863, t90042, t90262, t91012);
        let (t91199, t91228) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1788(t6587, t6622, t1042, t12787, t1715, t20795, t20809, t3711, t44190, t5340, t57471, t5819, t6429, t6640, t6690, t70758, t71275, t71513, t82816, t83504, t83539, t83558, t83580);
        let (t91260, t91272) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1789(t58777, t70942, t83699, t83719, t83731, t83735, t83748, t83751, t83758, t83783, t83798, t6573, t6587);
        let t91303 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1790(t24677, t467, t475, t484, t52, t6594, t6601, t71187, t71192, t83849, t83851, t83860, t83863, t83871, t83891, t83897, rho1);
        let t91352 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1791(t1774, t471, t1042, t1261, t12866, t1715, t17344, t17694, t1797, t20820, t24652, t24655, t24808, t3718, t3720, t5268, t5373, t5381, t6625, t82725, t82799, t83607, t83992, t83994, t88916, t90885);
        let t91378 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1792(t12787, t17693, t17694, t17729, t17747, t1785, t20956, t225, t24647, t24680, t3720, t480, t484, t5046, t59144, t71718, t71744, t84029, t84032, t84061, t84645, t89883, t90881);
    (t91012, t91037, t91060, t91119, t91173, t91199, t91228, t91260, t91272, t91303, t91352, t91378)
}
