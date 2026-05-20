//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1788;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1789;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta489<F: Float>(t342: F, t7135: F, t1071: F, t3140: F, t1078: F, t1982: F, t1976: F, t3057: F, t989: F, t11239: F, t378: F, t3143: F, t1035: F, t3046: F, t994: F, t11199: F, t1981: F, t7143: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25634, t25640, t25651, t25658, t25671, t25672) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1788::<F>(t342, t7135, t1071, t3140, t1078, t1982, t1976, t3057, t989, t11239, t378, t3143);
        let (t25681, t25692, t25695, t25698) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1789::<F>(t1035, t7135, t1976, t3046, t994, t11199, t1981);
        let t25699 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1790::<F>(t25698, t7143);
    (t25634, t25640, t25651, t25658, t25671, t25672, t25681, t25692, t25695, t25698, t25699)
}
