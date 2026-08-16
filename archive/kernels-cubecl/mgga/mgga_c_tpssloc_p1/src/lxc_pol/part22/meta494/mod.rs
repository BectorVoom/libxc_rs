//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta494<F: Float>(t21444: F, t340: F, t343: F, t974: F, t1597: F, t5836: F, t4546: F, t5842: F, t20217: F, t978: F, t977: F, t10217: F, t20234: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21446, t21447, t21452, t21453, t21456, t21458, t21459, t21462, t21463, t21468) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1921::<F>(t21444, t340, t343, t974, t1597, t5836, t4546, t5842, t20217, t978, t977, t10217, t20234);
    (t21446, t21447, t21452, t21453, t21456, t21458, t21459, t21462, t21463, t21468)
}
