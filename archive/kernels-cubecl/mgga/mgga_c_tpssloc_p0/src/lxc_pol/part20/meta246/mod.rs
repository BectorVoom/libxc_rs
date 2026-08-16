//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta246<F: Float>(t153: F, t9862: F, t2371: F, t2531: F, t2528: F, t2517: F, t607: F, t707: F, t2652: F, t2663: F, t181: F, t686: F, t781: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9863, t9864, t9865, t9866, t9867, t9868, t9869, t9870, t9871, t9872, t9874) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1362::<F>(t153, t9862, t2371, t2531, t2528, t2517, t607, t707, t2652, t2663, t181, t686, t781);
    (t9863, t9864, t9865, t9866, t9867, t9868, t9869, t9870, t9871, t9872, t9874)
}
