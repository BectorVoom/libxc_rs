//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2125;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2126;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta684<F: Float>(t1864: F, t5445: F, t2240: F, t5399: F, t3953: F, t3961: F, t3967: F, t1437: F, t4017: F, t72: F, t1433: F, t4021: F, t641: F, t19445: F, t79: F, t19299: F, t608: F, t3966: F, t2235: F, t17635: F, t605: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t96469, t96473, t96479, t96482, t96502, t96506) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2125::<F>(t1864, t5445, t2240, t5399, t3953, t3961, t3967, t1437, t4017, t72, t1433, t4021);
        let (t96517, t96521, t96535, t96553, t96556, t96559) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2126::<F>(t5445, t641, t72, t19445, t79, t19299, t608, t3966, t2235, t5399, t17635, t605);
    (t96469, t96473, t96479, t96482, t96502, t96506, t96517, t96521, t96535, t96553, t96556, t96559)
}
