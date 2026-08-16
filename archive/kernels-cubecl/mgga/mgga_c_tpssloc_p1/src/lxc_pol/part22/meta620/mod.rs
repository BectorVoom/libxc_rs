//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2151;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta620<F: Float>(t1214: F, t820: F, t3624: F, t52627: F, t43763: F, t44827: F, t3515: F, t4983: F, t49850: F, t11818: F, t1213: F, t248: F, t5012: F, t11820: F, t5019: F, t11791: F, t5024: F, t5002: F, t11153: F, t4899: F, t3540: F, t4961: F, t1227: F, t4973: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t52897, t52903, t52919, t52953, t52973) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2151::<F>(t1214, t820, t3624, t52627, t43763, t44827, t3515, t4983, t49850, t11818, t1213, t248, t5012);
        let (t52974, t52988, t52992, t52994, t52995, t53000, t53033) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2152::<F>(t52973, t11820, t5019, t11791, t5024, t5002, t11153, t4899, t3540, t4961, t1227, t4973, t49850);
    (t52897, t52903, t52919, t52953, t52974, t52988, t52992, t52994, t52995, t53000, t53033)
}
