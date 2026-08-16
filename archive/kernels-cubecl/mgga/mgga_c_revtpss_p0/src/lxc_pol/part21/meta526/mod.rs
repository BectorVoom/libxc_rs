//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2167;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta526<F: Float>(t16551: F, t342: F, t11631: F, t12050: F, t3151: F, t15907: F, t12077: F, t378: F, t3154: F, t12046: F, t357: F, t3133: F, t3302: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16552, t16554, t16555, t16558, t16559, t16561, t16562, t16565, t16566, t16568, t16569, t16573) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2167::<F>(t16551, t342, t11631, t12050, t3151, t15907, t12077, t378, t3154, t12046, t357, t3133, t3302);
    (t16552, t16554, t16555, t16558, t16559, t16561, t16562, t16565, t16566, t16568, t16569, t16573)
}
