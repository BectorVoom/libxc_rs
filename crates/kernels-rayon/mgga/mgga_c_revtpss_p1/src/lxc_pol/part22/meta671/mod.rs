//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2640;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2641;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta671(t20747: f64, t247: f64, t3719: f64, t369: f64, t6593: f64, t475: f64, t467: f64, t1260: f64, t17307: f64, t1256: f64, t6602: f64, t6595: f64, t6598: f64, t1266: f64, t17344: f64, t17396: f64, t17401: f64, t17721: f64, t17763: f64, t1808: f64, t3647: f64, t5270: f64, t5348: f64, t5354: f64, t5386: f64, t5391: f64, t6683: f64, t1248: f64, t6587: f64, t1250: f64, t3720: f64, t17183: f64, t5330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21267, t21271, t21272, t21275) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2640(t20747, t247, t3719, t369, t6593, t475, t467, t1260, t17307);
        let t21295 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2641(t1256, t6602, t6595, t6598, t1266, t17344, t17396, t17401, t17721, t17763, t1808, t21267, t21272, t21275, t3647, t5270, t5348, t5354, t5386, t5391, t6683);
        let (t21298, t21299, t21300, t21306) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2642(t1248, t6587, t1250, t3720, t17183, t5330);
    (t21267, t21271, t21272, t21275, t21295, t21298, t21299, t21300, t21306)
}
