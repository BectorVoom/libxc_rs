//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta359<F: Float>(t10595: F, t5698: F, t896: F, t4362: F, t4370: F, t2798: F, t5705: F, t10599: F, t4378: F, t2815: F, t10296: F, t10542: F, t10545: F, t10556: F, t13552: F, t13566: F, t13675: F, t13679: F, t17173: F, t17180: F, t17185: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17210, t17211, t17213, t17216, t17218, t17219, t17221, t17224, t17238) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1596::<F>(t10595, t5698, t896, t4362, t4370, t2798, t5705, t10599, t4378, t2815, t10296, t10542, t10545, t10556, t13552, t13566, t13675, t13679, t17173, t17180, t17185);
    (t17210, t17211, t17213, t17216, t17218, t17219, t17221, t17224, t17238)
}
