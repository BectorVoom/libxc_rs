//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta503<F: Float>(t11631: F, t12050: F, t3151: F, t15907: F, t12077: F, t378: F, t342: F, t3154: F, t12046: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16553, t16554, t16555, t16558, t16559, t16560, t16561, t16562, t16565, t16566) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2241::<F>(t11631, t12050, t3151, t15907, t12077, t378, t342, t3154, t12046);
    (t16553, t16554, t16555, t16558, t16559, t16560, t16561, t16562, t16565, t16566)
}
