//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1162;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1163;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta264<F: Float>(t2035: F, t7235: F, t2033: F, t531: F, t1353: F, t1450: F, t2014: F, t2022: F, t212: F, t1358: F, t689: F, t2023: F, t786: F, t1364: F, t533: F, t7021: F, t816: F, t1941: F, t540: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7236, t7237) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1162::<F>(t2035, t7235, t2033, t531);
        let (t7238, t7239, t7241, t7242) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1163::<F>(t1353, t1450, t7237, t2014, t2022, t212);
        let (t7243, t7245, t7246, t7248, t7251, t7252) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1164::<F>(t1358, t7242, t689, t2023, t786, t1364, t533, t7021, t816, t1941, t540);
    (t7236, t7237, t7238, t7239, t7241, t7242, t7243, t7245, t7246, t7248, t7251, t7252)
}
