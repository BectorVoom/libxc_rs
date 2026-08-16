//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta129<F: Float>(t374: F, t486: F, t677: F, t485: F, t1203: F, t1222: F, t221: F, t3426: F, t456: F, t1197: F, t135: F, t1174: F) -> (F, F, F, F, F) {
        let (t3540, t3542, t3543, t3547, t3549) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk688::<F>(t374, t486, t677, t485, t1203, t1222, t221, t3426, t456, t1197, t135, t1174);
    (t3540, t3542, t3543, t3547, t3549)
}
