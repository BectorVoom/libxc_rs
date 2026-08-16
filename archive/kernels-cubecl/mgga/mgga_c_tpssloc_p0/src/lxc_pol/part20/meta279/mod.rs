//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1463;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1464;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta279<F: Float>(t10588: F, t901: F, t276: F, t285: F, t2799: F, t896: F, t273: F, t10311: F, t10318: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10589: F, t10553: F, t942: F, t951: F, t959: F, t10544: F, t10530: F, t10538: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10591, t10595, t10596, t10597, t10599, t10600, t10602) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1463::<F>(t10588, t901, t276, t285, t2799, t896, t273, t10311, t10318, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575, t10589);
        let t10603 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1464::<F>(t10553, t10602);
        let (t10605, t10607, t10608, t10619) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1465::<F>(t10603, t942, t951, t959, t10544, t10530, t10538, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575);
    (t10591, t10595, t10596, t10597, t10599, t10600, t10603, t10605, t10607, t10608, t10619)
}
