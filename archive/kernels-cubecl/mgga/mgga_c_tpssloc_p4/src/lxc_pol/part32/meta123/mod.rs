//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk717;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk718;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk719;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta123<F: Float>(t3061: F, t61: F, t363: F, t368: F, t1017: F, t67: F, t1058: F, t1044: F, t820: F, t374: F, t376: F, t677: F, t370: F, t1032: F, t1036: F, t121: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3062, t3067, t3068) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk717::<F>(t3061, t61, t363, t368, t1017, t67);
        let (t3069, t3070) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk718::<F>(t3067, t3068, t1058);
        let t3071 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk719::<F>(t1044, t820);
        let (t3082, t3084, t3092, t3101) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk720::<F>(t374, t376, t677, t370, t1032, t1036, t121);
    (t3062, t3067, t3068, t3069, t3070, t3071, t3082, t3084, t3092, t3101)
}
