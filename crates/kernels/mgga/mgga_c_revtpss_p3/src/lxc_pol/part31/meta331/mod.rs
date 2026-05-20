//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1335;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1336;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta331<F: Float>(t3057: F, t3286: F, t1071: F, t1086: F, t994: F, t3316: F, t989: F, t11239: F, t11627: F, t342: F, t1129: F, t3431: F, t408: F, t3434: F, t421: F, t1126: F, t3432: F, t418: F, t240: F, t3698: F, t3361: F, t635: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12149, t12154, t12160, t12166, t12167, t12226) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1335::<F>(t3057, t3286, t1071, t1086, t994, t3316, t989, t11239, t11627, t342, t1129, t3431);
        let (t12227, t12230, t12243, t12248, t12254, t12256) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1336::<F>(t12226, t408, t3434, t421, t1126, t3432, t3431, t418, t240, t3698, t3361, t635);
    (t12149, t12154, t12160, t12166, t12167, t12227, t12230, t12243, t12248, t12254, t12256)
}
