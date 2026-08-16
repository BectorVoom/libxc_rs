//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta447<F: Float>(t1369: F, t22783: F, t3777: F, t6951: F, t6597: F, t6924: F, t281: F, t1307: F, t1361: F, t22690: F, t547: F, t6546: F) -> (F, F, F, F, F, F, F) {
        let (t22784, t22788, t22791, t22792, t22794, t22795, t22797) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1706::<F>(t1369, t22783, t3777, t6951, t6597, t6924, t281, t1307, t1361, t22690, t547, t6546);
    (t22784, t22788, t22791, t22792, t22794, t22795, t22797)
}
