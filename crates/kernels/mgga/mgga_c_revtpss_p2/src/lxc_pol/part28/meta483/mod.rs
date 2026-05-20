//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1834;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1835;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta483<F: Float>(t1071: F, t3140: F, t1078: F, t1982: F, t7135: F, t988: F, t7145: F, t1976: F, t3057: F, t989: F, t225: F, t25586: F, t385: F, t11239: F, t378: F, t3143: F, t3151: F, t3304: F, t3318: F, t7168: F, t1035: F, t1043: F, t1089: F, t3133: F, t3046: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25640, t25648, t25651) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1834::<F>(t1071, t3140, t1078, t1982, t7135, t988, t7145, t1976, t3057);
        let (t25658, t25662, t25671, t25674) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1835::<F>(t1976, t989, t225, t25586, t385, t11239, t378, t1078, t1982, t3143, t3151, t3304);
        let (t25678, t25681, t25683, t25687, t25692) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1836::<F>(t3151, t3318, t7168, t1035, t7135, t1043, t1089, t3133, t1976, t3046);
    (t25640, t25648, t25651, t25658, t25662, t25671, t25674, t25678, t25681, t25683, t25687, t25692)
}
