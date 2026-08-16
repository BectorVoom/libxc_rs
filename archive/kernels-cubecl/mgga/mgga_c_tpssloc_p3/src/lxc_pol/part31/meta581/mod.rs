//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1820;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta581<F: Float>(t26411: F, t6914: F, t22704: F, t22705: F, t5345: F, t22690: F, t552: F, t26447: F, t90607: F, t22751: F, t26397: F, t22892: F, t22893: F, t26396: F, t26384: F, t26388: F, t7733: F, t81186: F, t5318: F, t5187: F, t562: F, t26392: F, t80670: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90759, t90781, t90787, t90789, t90791, t90794) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1820::<F>(t26411, t6914, t22704, t22705, t5345, t22690, t552, t26447, t90607, t22751, t26397, t22892, t22893, t26396);
        let (t90797, t90805, t90807, t90809, t90818, t90837) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1821::<F>(t22892, t22893, t26384, t26388, t7733, t81186, t5318, t552, t5187, t562, t26392, t80670);
    (t90759, t90781, t90787, t90789, t90791, t90794, t90797, t90805, t90807, t90809, t90818, t90837)
}
