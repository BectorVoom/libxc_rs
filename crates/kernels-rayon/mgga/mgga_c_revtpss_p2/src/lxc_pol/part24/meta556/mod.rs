//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1661;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1662;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1663;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1664;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta556(t141: f64, t88083: f64, t930: f64, t88091: f64, t41329: f64, t63453: f64, t63459: f64, t63464: f64, t77499: f64, t77559: f64, t77561: f64, t88085: f64, t88089: f64, t88093: f64, t88097: f64, t51978: f64, t77505: f64, t77507: f64, t77509: f64, t88104: f64, t88108: f64, t88114: f64, t88118: f64, t88122: f64, t88126: f64, t88130: f64, t88134: f64, t916: f64, t6113: f64, t41401: f64, t2908: f64, t88132: f64, t41382: f64, t6120: f64, t2897: f64, t2880: f64, t41307: f64, t77736: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88168, t88171, t88188) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1661(t141, t88083, t930, t88091, t41329, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097);
        let t88201 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1662(t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t88202, t88203, t88206, t88209, t88211, t88214, t88216) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1663(t88188, t88201, t916, t6113, t41401, t141, t2908, t88132, t41382, t6120, t2897, t2880);
        let t88218 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1664(t41307, t51978, t77736, t88118, t88126, t88134, t88168, t88171, t88203, t88206, t88209, t88211, t88214, t88216);
    (t88168, t88171, t88202, t88203, t88206, t88209, t88211, t88214, t88216, t88218)
}
