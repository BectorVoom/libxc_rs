//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta308<F: Float>(t2775: F, t283: F, t135: F, t3142: F, t973: F, t3147: F, t3152: F, t248: F, t3101: F, t3132: F, t3130: F, t225: F, t3167: F) -> (F, F, F, F, F, F) {
        let (t10969, t10982, t10985, t10994, t11003, t11010) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1228::<F>(t2775, t283, t135, t3142, t973, t3147, t3152, t248, t3101, t3132, t3130, t225, t3167);
    (t10969, t10982, t10985, t10994, t11003, t11010)
}
