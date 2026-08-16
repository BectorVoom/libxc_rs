//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta129<F: Float>(t422: F, t3236: F, t1124: F, t1128: F, t1127: F, t432: F, t427: F, t3293: F, t435: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3314, t3315, t3319, t3327, t3331, t3332, t3339, t3346, t3355, t3356, t3357, t3358) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk734::<F>(t422, t3236, t1124, t1128, t1127, t432, t427, t3293, t435);
    (t3314, t3315, t3319, t3327, t3331, t3332, t3339, t3346, t3355, t3356, t3357, t3358)
}
