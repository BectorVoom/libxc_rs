//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta588<F: Float>(t22844: F, t6976: F, t5259: F, t80820: F, t80767: F, t80776: F, t22779: F, t26292: F, t80784: F, t80792: F, t80794: F, t16060: F, t6944: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91208, t91214, t91221, t91223, t91225, t91244, t91246, t91247, t91278) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1831::<F>(t22844, t6976, t5259, t80820, t80767, t80776, t22779, t26292, t80784, t80792, t80794, t16060, t6944);
    (t91208, t91214, t91221, t91223, t91225, t91244, t91246, t91247, t91278)
}
