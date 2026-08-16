//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1065;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1066;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1067;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta254<F: Float>(t1336: F, t6944: F, t1354: F, t1358: F, t2003: F, t552: F, t59: F, t240: F, t1369: F, t2010: F, t6883: F, t562: F) -> (F, F, F, F, F, F, F, F, F) {
        let t6945 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1065::<F>(t1336, t6944);
        let (t6946, t6948, t6950, t6951) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1066::<F>(t1354, t6945, t1358, t2003, t552, t59, t240);
        let t6952 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1067::<F>(t1336, t6951);
        let (t6953, t6966, t6968) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1068::<F>(t1369, t6952, t2010, t6883, t552, t562);
    (t6945, t6946, t6948, t6950, t6951, t6952, t6953, t6966, t6968)
}
