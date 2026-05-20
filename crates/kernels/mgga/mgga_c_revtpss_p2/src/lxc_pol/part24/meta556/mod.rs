//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1661;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1662;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1663;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1664;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta556<F: Float>(t141: F, t88083: F, t930: F, t88091: F, t41329: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t88085: F, t88089: F, t88093: F, t88097: F, t51978: F, t77505: F, t77507: F, t77509: F, t88104: F, t88108: F, t88114: F, t88118: F, t88122: F, t88126: F, t88130: F, t88134: F, t916: F, t6113: F, t41401: F, t2908: F, t88132: F, t41382: F, t6120: F, t2897: F, t2880: F, t41307: F, t77736: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t88168, t88171, t88188) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1661::<F>(t141, t88083, t930, t88091, t41329, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097);
        let t88201 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1662::<F>(t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t88202, t88203, t88206, t88209, t88211, t88214, t88216) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1663::<F>(t88188, t88201, t916, t6113, t41401, t141, t2908, t88132, t41382, t6120, t2897, t2880);
        let t88218 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1664::<F>(t41307, t51978, t77736, t88118, t88126, t88134, t88168, t88171, t88203, t88206, t88209, t88211, t88214, t88216);
    (t88168, t88171, t88202, t88203, t88206, t88209, t88211, t88214, t88216, t88218)
}
