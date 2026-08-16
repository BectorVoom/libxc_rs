//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1665;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1666;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1667;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta557(t23535: f64, t4598: f64, t18987: f64, t6120: f64, t4614: f64, t18979: f64, t11341: f64, t141: f64, t88116: f64, t88095: f64, t930: f64, t77804: f64, t88085: f64, t88093: f64, t88104: f64, t88108: f64, t88114: f64, t88122: f64, t88130: f64, t88202: f64, t923: f64, t2908: f64, t88124: f64, t88087: f64, t52128: f64, t63453: f64, t63459: f64, t63464: f64, t63533: f64, t63538: f64, t63545: f64, t77559: f64, t77561: f64, t77806: f64, t77858: f64, t88166: f64, t88218: f64, t41672: f64, t77499: f64, t77505: f64, t77507: f64, t77509: f64, t77663: f64, t77667: f64, t88089: f64, t88097: f64, t88144: f64, t88147: f64, t88150: f64, t88161: f64, t88164: f64, t41690: f64, t51978: f64, t77736: f64, t88118: f64, t88126: f64, t88134: f64, t88168: f64, t88171: f64, t88203: f64, t88206: f64, t88209: f64, t88211: f64, t88214: f64, t88216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88220, t88222, t88224, t88226, t88229, t88232, t88242) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1665(t23535, t4598, t18987, t6120, t4614, t18979, t11341, t141, t88116, t88095, t930, t77804, t88085, t88093, t88104, t88108, t88114, t88122, t88130);
        let (t88252, t88257, t88260, t88262) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1666(t88202, t923, t141, t2908, t88124, t88087, t930, t52128, t63453, t63459, t63464, t63533, t63538, t63545, t77559, t77561, t77806, t77858);
        let (t88264, t88291) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1667(t88166, t88218, t88242, t88262, t41672, t77499, t77505, t77507, t77509, t77663, t77667, t88089, t88097, t88144, t88147, t88150, t88161, t88164);
        let t88305 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1668(t41690, t51978, t77736, t88118, t88126, t88134, t88168, t88171, t88203, t88206, t88209, t88211, t88214, t88216);
    (t88220, t88222, t88224, t88226, t88229, t88232, t88252, t88257, t88260, t88264, t88291, t88305)
}
