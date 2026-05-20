//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1866;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta492<F: Float>(t2022: F, t4077: F, t25924: F, t4075: F, t7282: F, t1955: F, t1385: F) -> (F, F, F, F, F) {
        let (t25925, t25926, t25929, t25930) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1866::<F>(t2022, t4077, t25924, t4075, t7282, t1955);
        let t25931 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1867::<F>(t1385, t2022);
    (t25925, t25926, t25929, t25930, t25931)
}
