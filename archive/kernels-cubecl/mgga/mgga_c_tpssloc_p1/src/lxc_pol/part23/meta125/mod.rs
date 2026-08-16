//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta125<F: Float>(t1827: F, t3799: F, t1788: F, t588: F, t592: F, t546: F, t68: F, t1365: F, t1799: F, t1831: F, t3866: F, t1835: F, t225: F) -> (F, F, F, F, F, F, F) {
        let (t5255, t5264, t5266, t5278, t5279, t5306, t5321) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk623::<F>(t1827, t3799, t1788, t588, t592, t546, t68, t1365, t1799, t1831, t3866, t1835, t225);
    (t5255, t5264, t5266, t5278, t5279, t5306, t5321)
}
