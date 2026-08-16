//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1788;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1789;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta491<F: Float>(t25901: F, t25904: F, t212: F, t7274: F, t1358: F, t689: F, t2022: F, t785: F, t2439: F, t1032: F, t1419: F, t1955: F, t545: F, t9656: F, t4075: F, t7282: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25905, t25912, t25913, t25914, t25916, t25917, t25919, t25920, t25921) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1788::<F>(t25901, t25904, t212, t7274, t1358, t689, t2022, t785, t2439, t1032, t1419, t1955);
        let t25924 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1789::<F>(t545, t9656);
        let (t25929, t25930) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1790::<F>(t4075, t7282, t1955);
    (t25905, t25912, t25913, t25914, t25916, t25917, t25919, t25920, t25921, t25924, t25929, t25930)
}
