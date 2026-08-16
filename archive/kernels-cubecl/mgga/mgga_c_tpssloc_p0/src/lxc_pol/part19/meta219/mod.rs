//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta219<F: Float>(t10727: F, t2792: F, t2836: F, t2844: F, t912: F, t2842: F, t2880: F, t933: F, t10662: F, t913: F, t2860: F, t919: F) -> (F, F, F, F, F, F, F) {
        let (t10729, t10731, t10733, t10734, t10737, t10739, t10740) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk917::<F>(t10727, t2792, t2836, t2844, t912, t2842, t2880, t933, t10662, t913, t2860, t919);
    (t10729, t10731, t10733, t10734, t10737, t10739, t10740)
}
