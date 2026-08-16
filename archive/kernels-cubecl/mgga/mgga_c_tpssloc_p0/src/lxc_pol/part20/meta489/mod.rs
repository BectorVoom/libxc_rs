//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1981;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta489<F: Float>(t1307: F, t16195: F, t3719: F, t5279: F, t1347: F, t16018: F, t1345: F, t1348: F, t16176: F, t16186: F, t16192: F, t1819: F, t1821: F, t3839: F, t3844: F, t3847: F, t5272: F, t5278: F, t5280: F, t5283: F, t546: F, t548: F, t550: F, t1343: F, t820: F, t12365: F, t1827: F, t12300: F, t1799: F, t3734: F, t12351: F, t12418: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16196, t16199, t16202, t16205) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1981::<F>(t1307, t16195, t3719, t5279, t1347, t16018, t1345, t1348, t16176, t16186, t16192, t1819, t1821, t3839, t3844, t3847, t5272, t5278, t5280, t5283, t546, t548);
        let (t16206, t16208, t16211, t16214, t16217, t16224) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1982::<F>(t16205, t550, t1343, t820, t12365, t1827, t12300, t1799, t3734, t12351, t12418);
    (t16196, t16199, t16202, t16205, t16206, t16208, t16211, t16214, t16217, t16224)
}
