//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta396 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta396<F: Float>(t13615: F, t901: F, t2815: F, t4370: F, t896: F, t2807: F, t4378: F, t2798: F, t4362: F, t10595: F, t1547: F, t2799: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13616, t13623, t13624, t13626, t13629, t13630, t13632, t13634, t13635) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1789::<F>(t13615, t901, t2815, t4370, t896, t2807, t4378, t2798, t4362, t10595, t1547, t2799);
    (t13616, t13623, t13624, t13626, t13629, t13630, t13632, t13634, t13635)
}
