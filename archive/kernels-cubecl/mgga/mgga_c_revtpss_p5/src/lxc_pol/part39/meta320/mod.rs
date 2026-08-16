//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1094;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta320<F: Float>(t1063: F, t11988: F, t1062: F, t3196: F, t3223: F, t3229: F, t369: F, t361: F, t351: F, t3106: F, t3111: F, t3156: F, t3172: F, t3150: F, t1032: F, t3043: F, t1040: F, t1035: F, t11239: F, t342: F, t3145: F, t334: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11989, t11991, t11994, t12004, t12007, t12009) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1094::<F>(t1063, t11988, t1062, t3196, t3223, t3229, t369, t361, t351, t3106, t3111, t3156, t3172);
        let (t12010, t12021, t12046, t12047, t12050) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1095::<F>(t12009, t3150, t1032, t3043, t1040, t1035, t11239, t342, t3145, t334);
    (t11989, t11991, t11994, t12004, t12007, t12010, t12021, t12046, t12047, t12050)
}
