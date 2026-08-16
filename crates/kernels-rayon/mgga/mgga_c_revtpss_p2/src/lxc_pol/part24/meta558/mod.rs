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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta558(t77804: f64, t88085: f64, t88093: f64, t88104: f64, t88108: f64, t88114: f64, t88122: f64, t88130: f64, t88220: f64, t88222: f64, t88224: f64, t88226: f64, t88229: f64, t88232: f64, t52128: f64, t63453: f64, t63459: f64, t63464: f64, t63533: f64, t63538: f64, t63545: f64, t77559: f64, t77561: f64, t77806: f64, t77858: f64, t88252: f64, t88257: f64, t88260: f64, t6205: f64, t15421: f64, t23565: f64, t11299: f64, t88031: f64, t935: f64, t23550: f64, t52224: f64, t11452: f64, t11466: f64, t1622: f64, t19173: f64, t23714: f64, t23717: f64, t23776: f64, t2987: f64, t41238: f64, t41658: f64, t41667: f64, t4685: f64, t52642: f64, t52825: f64, t6158: f64, t6174: f64, t6177: f64, t64060: f64, t64319: f64, t78108: f64, t88008: f64, t88055: f64, t88140: f64, t88264: f64, t88291: f64, t88305: f64, t946: f64, t954: f64, t965: f64, t973: f64, t11387: f64, t41588: f64, t41592: f64, t77499: f64, t77505: f64, t77507: f64, t77509: f64, t77663: f64, t77667: f64, t88089: f64, t88097: f64, t88144: f64, t88147: f64, t88150: f64, t88161: f64, t88164: f64, t41610: f64, t51978: f64, t77736: f64, t88118: f64, t88126: f64, t88134: f64, t88168: f64, t88171: f64, t88203: f64, t88206: f64, t88209: f64, t88211: f64, t88214: f64, t88216: f64, t915: f64, t1609: f64, t23547: f64, t2874: f64, t2924: f64, t78329: f64, t11385: f64, t19255: f64, t6141: f64, t41520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t88321 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1669(t77804, t88085, t88093, t88104, t88108, t88114, t88122, t88130, t88220, t88222, t88224, t88226, t88229, t88232);
        let t88336 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1670(t52128, t63453, t63459, t63464, t63533, t63538, t63545, t77559, t77561, t77806, t77858, t88252, t88257, t88260);
        let (t88351, t88358, t88361, t88363, t88364) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1671(t6205, t15421, t23565, t11299, t88031, t935, t23550, t52224, t11452, t11466, t1622, t19173, t23714, t23717, t23776, t2987, t41238, t41658, t41667, t4685, t52642, t52825, t6158, t6174, t6177, t64060, t64319, t78108, t88008, t88055, t88140, t88264, t88291, t88305, t88321, t88336, t946, t954, t965, t973);
        let (t88368, t88382) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1672(t11387, t41588, t88031, t41592, t77499, t77505, t77507, t77509, t77663, t77667, t88089, t88097, t88144, t88147, t88150, t88161, t88164);
        let t88396 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1673(t41610, t51978, t77736, t88118, t88126, t88134, t88168, t88171, t88203, t88206, t88209, t88211, t88214, t88216);
        let t88412 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1674(t77804, t88085, t88093, t88104, t88108, t88114, t88122, t88130, t88220, t88222, t88224, t88226, t88229, t88232);
        let t88427 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1675(t52128, t63453, t63459, t63464, t63533, t63538, t63545, t77559, t77561, t77806, t77858, t88252, t88257, t88260);
        let (t88432, t88445, t88448, t88451) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1676(t88382, t88396, t88412, t88427, t915, t935, t1609, t23547, t2874, t2924, t78329, t11385, t19255, t6141);
        let (t88462, t88475) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1677(t41520, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097, t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
    (t88351, t88358, t88361, t88363, t88364, t88368, t88432, t88445, t88448, t88451, t88462, t88475)
}
