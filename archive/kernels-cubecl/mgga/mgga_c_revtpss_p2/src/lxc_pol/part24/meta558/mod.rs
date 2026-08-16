//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta558 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1669;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1670;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1671;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1672;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1673;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1674;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1675;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1676;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta558<F: Float>(t77804: F, t88085: F, t88093: F, t88104: F, t88108: F, t88114: F, t88122: F, t88130: F, t88220: F, t88222: F, t88224: F, t88226: F, t88229: F, t88232: F, t52128: F, t63453: F, t63459: F, t63464: F, t63533: F, t63538: F, t63545: F, t77559: F, t77561: F, t77806: F, t77858: F, t88252: F, t88257: F, t88260: F, t6205: F, t15421: F, t23565: F, t11299: F, t88031: F, t935: F, t23550: F, t52224: F, t11452: F, t11466: F, t1622: F, t19173: F, t23714: F, t23717: F, t23776: F, t2987: F, t41238: F, t41658: F, t41667: F, t4685: F, t52642: F, t52825: F, t6158: F, t6174: F, t6177: F, t64060: F, t64319: F, t78108: F, t88008: F, t88055: F, t88140: F, t88264: F, t88291: F, t88305: F, t946: F, t954: F, t965: F, t973: F, t11387: F, t41588: F, t41592: F, t77499: F, t77505: F, t77507: F, t77509: F, t77663: F, t77667: F, t88089: F, t88097: F, t88144: F, t88147: F, t88150: F, t88161: F, t88164: F, t41610: F, t51978: F, t77736: F, t88118: F, t88126: F, t88134: F, t88168: F, t88171: F, t88203: F, t88206: F, t88209: F, t88211: F, t88214: F, t88216: F, t915: F, t1609: F, t23547: F, t2874: F, t2924: F, t78329: F, t11385: F, t19255: F, t6141: F, t41520: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t88321 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1669::<F>(t77804, t88085, t88093, t88104, t88108, t88114, t88122, t88130, t88220, t88222, t88224, t88226, t88229, t88232);
        let t88336 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1670::<F>(t52128, t63453, t63459, t63464, t63533, t63538, t63545, t77559, t77561, t77806, t77858, t88252, t88257, t88260);
        let (t88351, t88358, t88361, t88363, t88364) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1671::<F>(t6205, t15421, t23565, t11299, t88031, t935, t23550, t52224, t11452, t11466, t1622, t19173, t23714, t23717, t23776, t2987, t41238, t41658, t41667, t4685, t52642, t52825, t6158, t6174, t6177, t64060, t64319, t78108, t88008, t88055, t88140, t88264, t88291, t88305, t88321, t88336, t946, t954, t965, t973);
        let (t88368, t88382) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1672::<F>(t11387, t41588, t88031, t41592, t77499, t77505, t77507, t77509, t77663, t77667, t88089, t88097, t88144, t88147, t88150, t88161, t88164);
        let t88396 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1673::<F>(t41610, t51978, t77736, t88118, t88126, t88134, t88168, t88171, t88203, t88206, t88209, t88211, t88214, t88216);
        let t88412 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1674::<F>(t77804, t88085, t88093, t88104, t88108, t88114, t88122, t88130, t88220, t88222, t88224, t88226, t88229, t88232);
        let t88427 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1675::<F>(t52128, t63453, t63459, t63464, t63533, t63538, t63545, t77559, t77561, t77806, t77858, t88252, t88257, t88260);
        let (t88432, t88445, t88448, t88451) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1676::<F>(t88382, t88396, t88412, t88427, t915, t935, t1609, t23547, t2874, t2924, t78329, t11385, t19255, t6141);
        let (t88462, t88475) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1677::<F>(t41520, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097, t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
    (t88351, t88358, t88361, t88363, t88364, t88368, t88432, t88445, t88448, t88451, t88462, t88475)
}
