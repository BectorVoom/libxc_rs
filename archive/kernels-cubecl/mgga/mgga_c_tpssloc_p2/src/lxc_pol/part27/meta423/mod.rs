//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1732;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta423<F: Float>(t191: F, t192: F, t3660: F, t2020: F, t2314: F, t6535: F, t12823: F, t1874: F, t4034: F, t6525: F, t12734: F, t671: F, t6862: F, t2006: F, t3752: F, t1323: F, t6955: F, t2015: F, t3888: F, t12021: F, t1887: F, t6916: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22607, t22608, t22610, t22612, t22614, t22616, t22618, t22619) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1732::<F>(t191, t192, t3660, t2020, t2314, t6535, t12823, t1874, t4034, t6525, t12734, t671, t6862);
        let (t22622, t22624, t22630, t22633) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1733::<F>(t2006, t3752, t1323, t6955, t2015, t3888, t12021, t1887, t6916);
    (t22607, t22608, t22610, t22612, t22614, t22616, t22618, t22619, t22622, t22624, t22630, t22633)
}
