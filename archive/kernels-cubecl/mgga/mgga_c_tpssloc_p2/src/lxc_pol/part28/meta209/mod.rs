//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta209<F: Float>(t4613: F, t4656: F, t349: F, t1626: F, t225: F, t1065: F, t1634: F, t3174: F, t1057: F, t4639: F, t1022: F, t3188: F) -> (F, F, F, F, F, F) {
        let (t4657, t4658, t4660, t4665, t4669, t4673) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk957::<F>(t4613, t4656, t349, t1626, t225, t1065, t1634, t3174, t1057, t4639, t1022, t3188);
    (t4657, t4658, t4660, t4665, t4669, t4673)
}
