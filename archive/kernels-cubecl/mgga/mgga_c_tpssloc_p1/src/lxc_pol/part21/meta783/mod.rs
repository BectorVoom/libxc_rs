//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta783 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta783<F: Float>(t39845: F, t54456: F, t39615: F, t39642: F, t39655: F, t39658: F, t39844: F, t57203: F, t57204: F, t57205: F, t57206: F, t57207: F, t57209: F, t57210: F, t57212: F, t57213: F, t57214: F) -> (F, F, F) {
        let (t57215, t57216, t57217) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2717::<F>(t39845, t54456, t39615, t39642, t39655, t39658, t39844, t57203, t57204, t57205, t57206, t57207, t57209, t57210, t57212, t57213, t57214);
    (t57215, t57216, t57217)
}
