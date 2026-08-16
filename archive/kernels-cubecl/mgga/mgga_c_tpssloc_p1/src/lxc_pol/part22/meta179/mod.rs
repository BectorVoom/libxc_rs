//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1071;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta179<F: Float>(t3815: F, t1788: F, t588: F, t592: F, t3829: F, t3833: F, t2426: F, t2486: F, t3819: F, t3821: F, t3825: F, t3827: F, t3832: F, t5169: F) -> (F, F, F, F, F, F, F, F) {
        let (t5263, t5264, t5265, t5266, t5267, t5268, t5269, t5270) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1071::<F>(t3815, t1788, t588, t592, t3829, t3833, t2426, t2486, t3819, t3821, t3825, t3827, t3832, t5169);
    (t5263, t5264, t5265, t5266, t5267, t5268, t5269, t5270)
}
