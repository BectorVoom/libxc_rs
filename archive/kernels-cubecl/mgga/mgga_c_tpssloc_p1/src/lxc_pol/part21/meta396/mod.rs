//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta396 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta396<F: Float>(t14363: F, t324: F, t2924: F, t4475: F, t10632: F, t1580: F, t2906: F, t10756: F, t10820: F, t13729: F, t14257: F, t14329: F, t14332: F, t14337: F, t14344: F, t1581: F, t2856: F, t2900: F, t2925: F, t2930: F, t2933: F, t4434: F, t4449: F, t4472: F, t924: F, t943: F, t952: F) -> (F, F, F, F, F) {
        let (t14364, t14366, t14369, t14370, t14373) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1872::<F>(t14363, t324, t2924, t4475, t10632, t1580, t2906, t10756, t10820, t13729, t14257, t14329, t14332, t14337, t14344, t1581, t2856, t2900, t2925, t2930, t2933, t4434, t4449, t4472, t924, t943, t952);
    (t14364, t14366, t14369, t14370, t14373)
}
