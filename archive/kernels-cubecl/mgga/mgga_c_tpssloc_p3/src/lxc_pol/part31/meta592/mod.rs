//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta592<F: Float>(t81375: F, t22724: F, t26344: F, t22643: F, t7691: F, t81195: F, t22573: F, t7684: F, t27240: F, t580: F, t1395: F, t7961: F) -> (F, F, F, F, F, F) {
        let (t91496, t91531, t91548, t91655, t91830, t91832) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1837::<F>(t81375, t22724, t26344, t22643, t7691, t81195, t22573, t7684, t27240, t580, t1395, t7961);
    (t91496, t91531, t91548, t91655, t91830, t91832)
}
