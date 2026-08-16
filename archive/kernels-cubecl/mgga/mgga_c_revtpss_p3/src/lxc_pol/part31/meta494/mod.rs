//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1804;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta494<F: Float>(t1385: F, t2022: F, t1426: F, t545: F, t7282: F, t10073: F, t2453: F, t7283: F, t136: F, t2029: F, t2457: F) -> (F, F, F, F, F, F, F, F) {
        let t25931 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1804::<F>(t1385, t2022);
        let (t25937, t25938, t25939, t25941, t25944, t25945, t25946) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1805::<F>(t1426, t545, t2022, t7282, t10073, t2453, t7283, t136, t2029, t2457);
    (t25931, t25937, t25938, t25939, t25941, t25944, t25945, t25946)
}
