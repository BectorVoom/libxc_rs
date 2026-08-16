//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2032;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2033;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2034;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta521(t21271: f64, t467: f64, t1260: f64, t17307: f64, t1256: f64, t6602: f64, t6595: f64, t6598: f64, t1266: f64, t17344: f64, t17396: f64, t17401: f64, t17721: f64, t17763: f64, t1808: f64, t21267: f64, t3647: f64, t5270: f64, t5348: f64, t5354: f64, t5386: f64, t5391: f64, t6683: f64, t1248: f64, t6587: f64, t1250: f64, t3720: f64, t17183: f64, t5330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t21272 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2032(t21271, t467);
        let t21275 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2033(t1260, t17307);
        let (t21283, t21285, t21287, t21295) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2034(t1256, t6602, t6595, t6598, t1266, t17344, t17396, t17401, t17721, t17763, t1808, t21267, t21272, t21275, t3647, t5270, t5348, t5354, t5386, t5391, t6683);
        let (t21298, t21299, t21300, t21306) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2035(t1248, t6587, t1250, t3720, t17183, t5330);
    (t21272, t21275, t21283, t21285, t21287, t21295, t21298, t21299, t21300, t21306)
}
