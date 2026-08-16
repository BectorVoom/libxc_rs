//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1039 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3630;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1039<F: Float>(t68707: F, t68709: F, t68711: F, t68714: F, t68716: F, t68718: F, t68723: F, t68725: F, t68727: F, t68730: F, t68733: F, t68735: F, t16677: F, t5192: F, t1196: F, t12485: F, t3524: F, t6534: F, t20400: F, t3535: F, t17164: F, t20391: F, t3531: F, t3427: F, t3433: F, t6439: F) -> (F, F, F, F, F, F, F) {
        let t68736 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3630::<F>(t68707, t68709, t68711, t68714, t68716, t68718, t68723, t68725, t68727, t68730, t68733, t68735);
        let (t68738, t68742, t68744, t68746, t68748, t68751) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3631::<F>(t16677, t5192, t1196, t12485, t3524, t6534, t20400, t3535, t17164, t20391, t3531, t3427, t3433, t6439);
    (t68736, t68738, t68742, t68744, t68746, t68748, t68751)
}
