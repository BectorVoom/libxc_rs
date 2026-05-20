//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1772;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1773;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1774;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta467<F: Float>(t225: F, t25286: F, t7048: F, t7071: F, t886: F, t7082: F, t72: F, t686: F, t7058: F, t2453: F, t7057: F, t136: F, t1958: F, t2457: F, t1954: F, t9645: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t25287, t25292, t25295, t25296) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1772::<F>(t225, t25286, t7048, t7071, t886, t7082, t72, t686);
        let (t25297, t25299, t25300, t25301) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1773::<F>(t25296, t7058, t2453, t7057, t136, t1958, t2457);
        let (t25303, t25304) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1774::<F>(t25299, t25301, t1954, t9645);
    (t25287, t25292, t25295, t25296, t25297, t25299, t25300, t25301, t25303, t25304)
}
