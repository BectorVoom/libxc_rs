//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1157;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta197<F: Float>(t2932: F, t5811: F, t959: F, t2980: F, t5392: F, t2979: F, t4514: F, t4531: F, t2994: F, t977: F, t5398: F, t978: F, t3003: F, t4384: F, t5718: F, t5721: F, t5724: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5812, t5814, t5817, t5818, t5821, t5824, t5825, t5828) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1157::<F>(t2932, t5811, t959, t2980, t5392, t2979, t4514, t4531, t2994, t977, t5398, t978);
        let (t5829, t5836) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1158::<F>(t5828, t977, t3003, t4384, t5718, t5721, t5724);
    (t5812, t5814, t5817, t5818, t5821, t5824, t5825, t5828, t5829, t5836)
}
