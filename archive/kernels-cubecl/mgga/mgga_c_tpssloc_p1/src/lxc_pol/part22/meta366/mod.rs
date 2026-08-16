//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta366<F: Float>(t5727: F, t912: F, t2792: F, t2844: F, t5726: F, t2842: F, t4395: F, t4399: F, t10704: F, t5694: F, t10702: F, t5743: F, t931: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17517, t17519, t17520, t17521, t17523, t17524, t17526, t17527, t17528, t17530, t17535) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1614::<F>(t5727, t912, t2792, t2844, t5726, t2842, t4395, t4399, t10704, t5694, t10702, t5743, t931);
    (t17517, t17519, t17520, t17521, t17523, t17524, t17526, t17527, t17528, t17530, t17535)
}
