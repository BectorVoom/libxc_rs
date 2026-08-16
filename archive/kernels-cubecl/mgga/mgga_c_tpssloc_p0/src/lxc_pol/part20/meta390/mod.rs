//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1767;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1768;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta390<F: Float>(t4351: F, t892: F, t914: F, t2837: F, t4354: F, t1543: F, t2841: F, t2845: F, t10650: F, t1557: F, t2787: F, t4396: F, t2770: F, t3966: F, t607: F, t2826: F, t136: F, t2250: F, t4337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13515, t13517, t13519, t13520, t13522, t13524, t13526) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1767::<F>(t4351, t892, t914, t2837, t4354, t1543, t2841, t2845, t10650, t1557, t2787, t4396);
        let (t13527, t13528) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1768::<F>(t2770, t3966, t607);
        let (t13529, t13530, t13532) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1769::<F>(t13528, t2826, t136, t2250, t4337);
    (t13515, t13517, t13519, t13520, t13522, t13524, t13526, t13527, t13528, t13529, t13530, t13532)
}
