//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1665;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1666;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1667;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta557<F: Float>(t23535: F, t4598: F, t18987: F, t6120: F, t4614: F, t18979: F, t11341: F, t141: F, t88116: F, t88095: F, t930: F, t77804: F, t88085: F, t88093: F, t88104: F, t88108: F, t88114: F, t88122: F, t88130: F, t88202: F, t923: F, t2908: F, t88124: F, t88087: F, t52128: F, t63453: F, t63459: F, t63464: F, t63533: F, t63538: F, t63545: F, t77559: F, t77561: F, t77806: F, t77858: F, t88166: F, t88218: F, t41672: F, t77499: F, t77505: F, t77507: F, t77509: F, t77663: F, t77667: F, t88089: F, t88097: F, t88144: F, t88147: F, t88150: F, t88161: F, t88164: F, t41690: F, t51978: F, t77736: F, t88118: F, t88126: F, t88134: F, t88168: F, t88171: F, t88203: F, t88206: F, t88209: F, t88211: F, t88214: F, t88216: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t88220, t88222, t88224, t88226, t88229, t88232, t88242) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1665::<F>(t23535, t4598, t18987, t6120, t4614, t18979, t11341, t141, t88116, t88095, t930, t77804, t88085, t88093, t88104, t88108, t88114, t88122, t88130);
        let (t88252, t88257, t88260, t88262) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1666::<F>(t88202, t923, t141, t2908, t88124, t88087, t930, t52128, t63453, t63459, t63464, t63533, t63538, t63545, t77559, t77561, t77806, t77858);
        let (t88264, t88291) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1667::<F>(t88166, t88218, t88242, t88262, t41672, t77499, t77505, t77507, t77509, t77663, t77667, t88089, t88097, t88144, t88147, t88150, t88161, t88164);
        let t88305 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1668::<F>(t41690, t51978, t77736, t88118, t88126, t88134, t88168, t88171, t88203, t88206, t88209, t88211, t88214, t88216);
    (t88220, t88222, t88224, t88226, t88229, t88232, t88252, t88257, t88260, t88264, t88291, t88305)
}
