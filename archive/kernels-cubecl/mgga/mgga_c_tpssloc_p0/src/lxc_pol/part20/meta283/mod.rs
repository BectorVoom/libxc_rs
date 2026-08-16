//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1477;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta283<F: Float>(t2843: F, t290: F, t10662: F, t10702: F, t10524: F, t2929: F, t951: F, t959: F, t2904: F, t2925: F, t950: F, t2880: F, t2888: F, t931: F) -> (F, F, F, F, F, F, F, F) {
        let (t10704, t10705, t10707, t10709, t10711, t10713, t10715, t10717) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1477::<F>(t2843, t290, t10662, t10702, t10524, t2929, t951, t959, t2904, t2925, t950, t2880, t2888, t931);
    (t10704, t10705, t10707, t10709, t10711, t10713, t10715, t10717)
}
