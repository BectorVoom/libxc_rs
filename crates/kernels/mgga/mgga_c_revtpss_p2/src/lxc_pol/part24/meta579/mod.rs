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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1785;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1786;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1787;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1788;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1789;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1790;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1791;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta579<F: Float>(t3628: F, t5825: F, t6573: F, t1235: F, t17709: F, t1791: F, t20851: F, t20956: F, t21063: F, t24636: F, t371: F, t372: F, t3720: F, t44844: F, t482: F, t5327: F, t6611: F, t6647: F, t70263: F, t70278: F, t70578: F, t83109: F, t84098: F, t84636: F, t89960: F, t1012: F, t1222: F, t1225: F, t1782: F, t21213: F, t21306: F, t24736: F, t24821: F, t24827: F, t24831: F, t24836: F, t3699: F, t44348: F, t44919: F, t5373: F, t57707: F, t6653: F, t83962: F, t87107: F, t87126: F, t87145: F, t1261: F, t12855: F, t13100: F, t17475: F, t21040: F, t24228: F, t24535: F, t247: F, t24792: F, t3604: F, t3625: F, t3626: F, t44225: F, t5312: F, t5381: F, t83392: F, t83394: F, t83435: F, t89822: F, t89826: F, t89863: F, t90042: F, t90262: F, t6587: F, t6622: F, t1042: F, t12787: F, t1715: F, t20795: F, t20809: F, t3711: F, t44190: F, t5340: F, t57471: F, t5819: F, t6429: F, t6640: F, t6690: F, t70758: F, t71275: F, t71513: F, t82816: F, t83504: F, t83539: F, t83558: F, t83580: F, t58777: F, t70942: F, t83699: F, t83719: F, t83731: F, t83735: F, t83748: F, t83751: F, t83758: F, t83783: F, t83798: F, t24677: F, t467: F, t475: F, t484: F, t52: F, t6594: F, t6601: F, t71187: F, t71192: F, t83849: F, t83851: F, t83860: F, t83863: F, t83871: F, t83891: F, t83897: F, rho1: F, t1774: F, t471: F, t12866: F, t17344: F, t17694: F, t1797: F, t20820: F, t24652: F, t24655: F, t24808: F, t3718: F, t5268: F, t6625: F, t82725: F, t82799: F, t83607: F, t83992: F, t83994: F, t88916: F, t90885: F, t17693: F, t17729: F, t17747: F, t1785: F, t225: F, t24647: F, t24680: F, t480: F, t5046: F, t59144: F, t71718: F, t71744: F, t84029: F, t84032: F, t84061: F, t84645: F, t89883: F, t90881: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t91012, t91037, t91060) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1785::<F>(t3628, t5825, t6573, t1235, t17709, t1791, t20851, t20956, t21063, t24636, t371, t372, t3720, t44844, t482, t5327, t6611, t6647, t70263, t70278, t70578, t83109, t84098, t84636, t89960);
        let t91119 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1786::<F>(t1012, t1222, t1225, t1782, t21213, t21306, t24736, t24821, t24827, t24831, t24836, t3699, t44348, t44919, t5373, t57707, t6653, t83962, t87107, t87126, t87145);
        let t91173 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1787::<F>(t1222, t1261, t12855, t13100, t17475, t21040, t24228, t24535, t247, t24792, t3604, t3625, t3626, t3720, t44225, t5312, t5381, t83392, t83394, t83435, t89822, t89826, t89863, t90042, t90262, t91012);
        let (t91199, t91228) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1788::<F>(t6587, t6622, t1042, t12787, t1715, t20795, t20809, t3711, t44190, t5340, t57471, t5819, t6429, t6640, t6690, t70758, t71275, t71513, t82816, t83504, t83539, t83558, t83580);
        let (t91260, t91272) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1789::<F>(t58777, t70942, t83699, t83719, t83731, t83735, t83748, t83751, t83758, t83783, t83798, t6573, t6587);
        let t91303 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1790::<F>(t24677, t467, t475, t484, t52, t6594, t6601, t71187, t71192, t83849, t83851, t83860, t83863, t83871, t83891, t83897, rho1);
        let t91352 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1791::<F>(t1774, t471, t1042, t1261, t12866, t1715, t17344, t17694, t1797, t20820, t24652, t24655, t24808, t3718, t3720, t5268, t5373, t5381, t6625, t82725, t82799, t83607, t83992, t83994, t88916, t90885);
        let t91378 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1792::<F>(t12787, t17693, t17694, t17729, t17747, t1785, t20956, t225, t24647, t24680, t3720, t480, t484, t5046, t59144, t71718, t71744, t84029, t84032, t84061, t84645, t89883, t90881);
    (t91012, t91037, t91060, t91119, t91173, t91199, t91228, t91260, t91272, t91303, t91352, t91378)
}
