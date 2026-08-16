//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta149 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta149<F: Float>(t3540: F, t485: F, t1203: F, t1222: F, t221: F, t3426: F, t456: F, t1197: F, t135: F, t1174: F, t1196: F, t2250: F) -> (F, F, F, F, F, F, F) {
        let (t3542, t3543, t3545, t3547, t3548, t3549, t3551) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk955::<F>(t3540, t485, t1203, t1222, t221, t3426, t456, t1197, t135, t1174, t1196, t2250);
    (t3542, t3543, t3545, t3547, t3548, t3549, t3551)
}
