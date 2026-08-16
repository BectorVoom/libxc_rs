//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1650;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1651;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1652;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta348<F: Float>(t12049: F, t12095: F, t12119: F, t12144: F, t225: F, t1995: F, t68: F, t1307: F, t3734: F, t1365: F, t3719: F, t12012: F, t1347: F, t1345: F, t1348: F, t3839: F, t3844: F, t3847: F, t5278: F, t546: F, t548: F, t550: F, t1380: F, t1372: F, t3787: F, t3793: F, t1351: F, t3791: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12147, t12155, t12156) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1650::<F>(t12049, t12095, t12119, t12144, t225, t1995, t68, t1307, t3734);
        let (t12157, t12161, t12164, t12167) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1651::<F>(t12155, t12156, t1307, t1365, t3719, t12012, t1347, t12147, t1345, t1348, t3839, t3844, t3847, t5278, t546, t548);
        let t12168 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1652::<F>(t12167, t550);
        let (t12169, t12172, t12177, t12178) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1653::<F>(t12168, t1380, t1372, t3787, t3793, t1351, t3791, t550);
    (t12147, t12156, t12157, t12161, t12164, t12167, t12168, t12169, t12172, t12177, t12178)
}
