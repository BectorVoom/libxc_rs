//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta203<F: Float>(t10266: F, t10357: F, t225: F, t68: F, t369: F, t10195: F, t2979: F, t1031: F, t3077: F, t1036: F, t3078: F, t1032: F, t3082: F) -> (F, F, F, F, F, F, F, F) {
        let (t10358, t10359, t10360, t10361, t10364, t10367, t10370, t10372) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk874::<F>(t10266, t10357, t225, t68, t369, t10195, t2979, t1031, t3077, t1036, t3078, t1032, t3082);
    (t10358, t10359, t10360, t10361, t10364, t10367, t10370, t10372)
}
