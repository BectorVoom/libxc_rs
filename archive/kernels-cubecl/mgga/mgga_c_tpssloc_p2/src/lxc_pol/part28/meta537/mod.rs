//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1796;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta537<F: Float>(t22715: F, t6551: F, t6640: F, t117: F, t4179: F, t6559: F, t22893: F, t23036: F, t229: F, t268: F, t22988: F, t23110: F, t23154: F, t23164: F, t234: F, t2710: F, t23176: F, t23185: F, t131: F, t2587: F, t81142: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t81632, t81633, t81640, t81642, t81651) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1796::<F>(t22715, t6551, t6640, t117, t4179, t6559, t22893, t23036, t229, t268);
        let (t81653, t81656, t81658, t81670, t81686) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1797::<F>(t22988, t23110, t81651, t22893, t23154, t23164, t234, t2710, t23176, t23185, t131, t2587, t81142);
    (t81632, t81633, t81640, t81642, t81651, t81653, t81656, t81658, t81670, t81686)
}
