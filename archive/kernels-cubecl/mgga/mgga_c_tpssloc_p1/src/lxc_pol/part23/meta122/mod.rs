//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta122 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta122<F: Float>(t5154: F, t763: F, t1787: F, t67: F, t758: F, t193: F, t533: F, t1845: F, t3701: F, t750: F) -> (F, F, F, F, F, F) {
        let (t5155, t5157, t5158, t5160, t5161, t5168) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk617::<F>(t5154, t763, t1787, t67, t758, t193, t533, t1845, t3701, t750);
    (t5155, t5157, t5158, t5160, t5161, t5168)
}
