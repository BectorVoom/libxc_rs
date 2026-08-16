//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta747 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2618;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta747<F: Float>(t3566: F, t5023: F, t15734: F, t3490: F, t11789: F, t1227: F, t248: F, t4733: F, t11712: F, t11913: F, t491: F, t11887: F, t52834: F, t11880: F, t15831: F, t225: F, t11605: F, t1760: F, t15816: F, t15908: F, t9467: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53507, t53515, t53519, t53545, t53565) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2618::<F>(t3566, t5023, t15734, t3490, t11789, t1227, t248, t4733, t11712, t11913, t491, t11887, t52834);
        let (t53592, t53613, t53646, t53658, t53677, t53703, t53777) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2619::<F>(t11913, t52834, t11880, t11712, t11887, t491, t15831, t225, t11605, t1760, t15816, t15908, t9467);
    (t53507, t53515, t53519, t53545, t53565, t53592, t53613, t53646, t53658, t53677, t53703, t53777)
}
