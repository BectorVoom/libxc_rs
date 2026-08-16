//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1584;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta414<F: Float>(t22779: F, t6937: F, t6950: F, t835: F, t1336: F, t1369: F, t3876: F, t6952: F, t3777: F, t6951: F, t6597: F, t6924: F, t281: F, t1307: F, t1361: F, t22690: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22780, t22782, t22783) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1584::<F>(t22779, t6937, t6950, t835, t1336);
        let (t22784, t22785, t22786, t22788, t22789, t22791, t22792, t22794) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1585::<F>(t1369, t22783, t3876, t6952, t3777, t6951, t6597, t6924, t281, t1307, t1361, t22690);
    (t22780, t22782, t22783, t22784, t22785, t22786, t22788, t22789, t22791, t22792, t22794)
}
