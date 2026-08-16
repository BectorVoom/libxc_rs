//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta666 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta666<F: Float>(t1193: F, t27506: F, t7378: F, t11153: F, t491: F, t24826: F, t27537: F, t27526: F, t86094: F, t24660: F, t24850: F, t24667: F) -> (F, F, F, F, F, F, F) {
        let (t94909, t94911, t94920, t94941, t94947, t94948, t94954) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2098::<F>(t1193, t27506, t7378, t11153, t491, t24826, t27537, t27526, t86094, t24660, t24850, t24667);
    (t94909, t94911, t94920, t94941, t94947, t94948, t94954)
}
