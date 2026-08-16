//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1762;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta431<F: Float>(t16558: F, t31: F, t65: F, t5399: F, t628: F, t1426: F, t3961: F, t3967: F, t1410: F, t3997: F, t1434: F, t19322: F, t19323: F, t19326: F, t19331: F, t3962: F, t5393: F, t5400: F, t5403: F, t642: F, t80: F) -> (F, F, F, F, F, F, F) {
        let (t19334, t19335, t19338, t19343, t19346, t19349, t19356) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1762::<F>(t16558, t31, t65, t5399, t628, t1426, t3961, t3967, t1410, t3997, t1434, t19322, t19323, t19326, t19331, t3962, t5393, t5400, t5403, t642, t80);
    (t19334, t19335, t19338, t19343, t19346, t19349, t19356)
}
