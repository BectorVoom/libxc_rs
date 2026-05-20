//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta166 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk820;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk821;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk822;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk823;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk824;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk825;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta166<F: Float>(t1774: F, t1211: F, t1828: F, t1277: F, t3579: F, t5044: F, t6423: F, t6427: F, t6431: F, t1477: F, t476: F, t52: F, t475: F, t467: F, t1785: F, t1803: F, t225: F, t6564: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t6573 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk820::<F>(t1774);
        let t6574 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk821::<F>(t1211, t6573);
        let t6580 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk822::<F>(t1774, t1828, t1277);
        let t6587 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk823::<F>(t3579, t5044, t6423, t6427, t6431);
        let (t6588, t6593) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk824::<F>(t1211, t6587, t1477, t476, t52);
        let t6594 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk825::<F>(t475, t6593);
        let (t6595, t6598, t6601) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk826::<F>(t467, t6594, t1785, t1803, t225, t6564);
    (t6573, t6574, t6580, t6587, t6588, t6593, t6594, t6595, t6598, t6601)
}
