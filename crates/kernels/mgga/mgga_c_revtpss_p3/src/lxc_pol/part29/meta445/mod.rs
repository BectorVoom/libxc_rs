//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1663;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1664;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1665;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1666;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1667;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta445<F: Float>(t25282: F, t2736: F, t2453: F, t7057: F, t1954: F, t9645: F, t1032: F, t860: F, t867: F, t786: F, t11007: F, t233: F, t7063: F, t251: F, t2769: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25283, t25299, t25304) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1663::<F>(t25282, t2736, t2453, t7057, t1954, t9645);
        let (t25305, t25308, t25309) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1664::<F>(t25304, t7057, t1032, t860, t867);
        let (t25310, t25317) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1665::<F>(t25309, t786, t11007, t233);
        let (t25365, t25372) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1666::<F>(t25309, t7063, t251, t786);
        let (t25373, t25374) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1667::<F>(t1032, t2769, t233);
    (t25283, t25299, t25304, t25305, t25308, t25309, t25310, t25317, t25365, t25372, t25373, t25374)
}
