//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1167;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta367<F: Float>(t44178: F, t43689: F, t440: F, t43776: F, t43819: F, t3330: F, t3355: F, t427: F, t457: F, t625: F, t221: F, t456: F, t461: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44179, t44223, t44249, t44275, t44320, t44348, t44361, t44466, t44483, t44487) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1167::<F>(t44178, t43689, t440, t43776, t43819, t3330, t3355, t427, t457, t625, t221, t456, t461);
    (t44179, t44223, t44249, t44275, t44320, t44348, t44361, t44466, t44483, t44487)
}
