//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1868;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1869;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta493<F: Float>(t1398: F, t1444: F, t543: F, t25931: F, t1426: F, t545: F, t2022: F, t7282: F, t10073: F, t2453: F, t7283: F, t136: F, t2029: F, t2457: F, t25920: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25933, t25934, t25937, t25938, t25939, t25941, t25944, t25945) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1868::<F>(t1398, t1444, t543, t25931, t1426, t545, t2022, t7282, t10073, t2453, t7283, t136, t2029);
        let t25946 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1869::<F>(t2457, t25945);
        let (t25948, t25949) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1870::<F>(t25944, t25946, t1426, t25920);
    (t25933, t25934, t25937, t25938, t25939, t25941, t25944, t25945, t25946, t25948, t25949)
}
