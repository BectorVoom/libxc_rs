//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2032;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2033;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2034;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta521<F: Float>(t21271: F, t467: F, t1260: F, t17307: F, t1256: F, t6602: F, t6595: F, t6598: F, t1266: F, t17344: F, t17396: F, t17401: F, t17721: F, t17763: F, t1808: F, t21267: F, t3647: F, t5270: F, t5348: F, t5354: F, t5386: F, t5391: F, t6683: F, t1248: F, t6587: F, t1250: F, t3720: F, t17183: F, t5330: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t21272 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2032::<F>(t21271, t467);
        let t21275 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2033::<F>(t1260, t17307);
        let (t21283, t21285, t21287, t21295) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2034::<F>(t1256, t6602, t6595, t6598, t1266, t17344, t17396, t17401, t17721, t17763, t1808, t21267, t21272, t21275, t3647, t5270, t5348, t5354, t5386, t5391, t6683);
        let (t21298, t21299, t21300, t21306) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2035::<F>(t1248, t6587, t1250, t3720, t17183, t5330);
    (t21272, t21275, t21283, t21285, t21287, t21295, t21298, t21299, t21300, t21306)
}
