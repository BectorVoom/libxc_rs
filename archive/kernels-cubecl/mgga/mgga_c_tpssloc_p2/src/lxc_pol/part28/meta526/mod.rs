//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta526<F: Float>(t12328: F, t2003: F, t12248: F, t59: F, t12267: F, t6944: F, t1336: F, t2690: F, t6943: F, t1354: F, t22770: F, t22779: F) -> (F, F, F, F, F, F) {
        let (t80899, t80901, t80910, t80914, t80915, t80920) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1777::<F>(t12328, t2003, t12248, t59, t12267, t6944, t1336, t2690, t6943, t1354, t22770, t22779);
    (t80899, t80901, t80910, t80914, t80915, t80920)
}
