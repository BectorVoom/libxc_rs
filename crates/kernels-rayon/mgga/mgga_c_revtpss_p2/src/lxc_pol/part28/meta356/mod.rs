//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1376;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta356(t1043: f64, t3153: f64, t3133: f64, t4982: f64, t3046: f64, t3286: f64, t3057: f64, t1071: f64, t1086: f64, t994: f64, t3316: f64, t989: f64, t11239: f64, t11627: f64, t342: f64, t1129: f64, t3431: f64, t408: f64, t3434: f64, t421: f64, t1130: f64, t3376: f64, t1126: f64, t3432: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12131, t12132, t12146, t12149, t12154, t12160) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1376(t1043, t3153, t3133, t4982, t3046, t3286, t3057, t1071, t1086, t994, t3316, t989);
        let (t12166, t12167, t12227, t12230, t12238, t12243) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1377(t11239, t11627, t342, t1129, t3431, t408, t3434, t421, t1130, t3376, t1126, t3432);
    (t12131, t12132, t12146, t12149, t12154, t12160, t12166, t12167, t12227, t12230, t12238, t12243)
}
