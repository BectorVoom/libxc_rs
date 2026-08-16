//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta205 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta205<F: Float>(t1294: F, t9905: F, t9892: F, t3684: F, t9467: F, t9882: F, t9888: F, t9885: F, t3824: F, t588: F, t1287: F, t2225: F) -> (F, F, F, F, F, F, F, F) {
        let (t12103, t12105, t12109, t12114, t12116, t12118, t12121, t12123) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk846::<F>(t1294, t9905, t9892, t3684, t9467, t9882, t9888, t9885, t3824, t588, t1287, t2225);
    (t12103, t12105, t12109, t12114, t12116, t12118, t12121, t12123)
}
