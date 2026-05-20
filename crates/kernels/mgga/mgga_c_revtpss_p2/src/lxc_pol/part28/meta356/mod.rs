//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1376;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta356<F: Float>(t1043: F, t3153: F, t3133: F, t4982: F, t3046: F, t3286: F, t3057: F, t1071: F, t1086: F, t994: F, t3316: F, t989: F, t11239: F, t11627: F, t342: F, t1129: F, t3431: F, t408: F, t3434: F, t421: F, t1130: F, t3376: F, t1126: F, t3432: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12131, t12132, t12146, t12149, t12154, t12160) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1376::<F>(t1043, t3153, t3133, t4982, t3046, t3286, t3057, t1071, t1086, t994, t3316, t989);
        let (t12166, t12167, t12227, t12230, t12238, t12243) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1377::<F>(t11239, t11627, t342, t1129, t3431, t408, t3434, t421, t1130, t3376, t1126, t3432);
    (t12131, t12132, t12146, t12149, t12154, t12160, t12166, t12167, t12227, t12230, t12238, t12243)
}
