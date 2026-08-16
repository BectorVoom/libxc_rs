//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta230 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta230<F: Float>(t300: F, t6091: F, t6064: F, t1703: F, t4869: F, t1156: F, t3375: F, t6068: F, t1164: F, t1147: F, t6084: F, t3400: F) -> (F, F, F, F, F, F, F, F) {
        let (t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk974::<F>(t300, t6091, t6064, t1703, t4869, t1156, t3375, t6068, t1164, t1147, t6084, t3400);
    (t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105)
}
