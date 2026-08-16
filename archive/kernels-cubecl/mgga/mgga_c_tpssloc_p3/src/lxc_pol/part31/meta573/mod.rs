//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta573<F: Float>(t25325: F, t6547: F, t23185: F, t25045: F, t82074: F, t6562: F, t6572: F, t86893: F, t23171: F, t23228: F, t7488: F, t214: F, t4265: F) -> (F, F, F, F, F) {
        let (t87733, t87753, t87776, t87779, t87782) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1808::<F>(t25325, t6547, t23185, t25045, t82074, t6562, t6572, t86893, t23171, t23228, t7488, t214, t4265);
    (t87733, t87753, t87776, t87779, t87782)
}
