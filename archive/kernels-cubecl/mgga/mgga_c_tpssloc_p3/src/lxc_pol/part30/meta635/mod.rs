//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2044;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta635<F: Float>(t25192: F, t81651: F, t82074: F, t225: F, t25220: F, t25054: F, t23030: F, t25205: F, t23164: F, t7479: F, t82133: F, t23204: F, t25216: F, t6562: F, t1519: F, t212: F, t23171: F, t6554: F, t25040: F, t87712: F, t25193: F, t81591: F, t10143: F, t7540: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87836, t87837, t87874, t87898, t87902, t87910) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2044::<F>(t25192, t81651, t82074, t225, t25220, t25054, t23030, t25205, t23164, t7479, t82133, t23204, t25216, t6562);
        let (t87911, t87915, t87927, t87932, t87975) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2045::<F>(t87910, t1519, t212, t23171, t6554, t25040, t82074, t87712, t25193, t81591, t10143, t7540);
    (t87836, t87837, t87874, t87898, t87902, t87911, t87915, t87927, t87932, t87975)
}
