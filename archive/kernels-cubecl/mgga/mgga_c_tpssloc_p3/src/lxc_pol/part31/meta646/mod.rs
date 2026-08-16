//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1918;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta646<F: Float>(t16662: F, t6552: F, t6553: F, t6554: F, t23164: F, t23204: F, t28276: F, t16968: F, t87052: F, t87053: F, t16887: F, t87057: F, t28342: F, t81979: F, t17022: F, t1880: F, t1894: F, t214: F, t252: F, t5527: F, t25038: F, t6646: F, t829: F, t28333: F, t6562: F, t794: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98319, t98322, t98325, t98328) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1918::<F>(t16662, t6552, t6553, t6554, t23164, t23204, t28276, t16968, t87052, t87053, t16887, t87057);
        let (t98330, t98334, t98336, t98339, t98342) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1919::<F>(t28342, t81979, t17022, t1880, t1894, t214, t252, t5527, t25038, t6646, t829, t28333, t6562, t794);
    (t98319, t98322, t98325, t98328, t98330, t98334, t98336, t98339, t98342)
}
