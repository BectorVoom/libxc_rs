//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1754;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1755;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta479<F: Float>(t25299: F, t25301: F, t1954: F, t9645: F, t7057: F, t1032: F, t860: F, t867: F, t786: F, t7060: F, t11007: F, t233: F, t213: F, t7048: F, t2470: F, t7059: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25303, t25304) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1754::<F>(t25299, t25301, t1954, t9645);
        let (t25305, t25307, t25308, t25309, t25310, t25311, t25317) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1755::<F>(t25304, t7057, t25301, t1032, t860, t867, t786, t7060, t11007, t233);
        let (t25322, t25331) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1756::<F>(t213, t7048, t2470, t7059);
    (t25303, t25304, t25305, t25307, t25308, t25309, t25310, t25311, t25317, t25322, t25331)
}
