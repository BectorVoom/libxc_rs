//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta352<F: Float>(t10216: F, t10969: F, t135: F, t4608: F, t973: F, t10868: F, t1539: F, t248: F, t1041: F, t1009: F, t4552: F, t1011: F) -> (F, F, F, F, F, F, F) {
        let (t14187, t14192, t14194, t14202, t14203, t14205, t14206) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1392::<F>(t10216, t10969, t135, t4608, t973, t10868, t1539, t248, t1041, t1009, t4552, t1011);
    (t14187, t14192, t14194, t14202, t14203, t14205, t14206)
}
