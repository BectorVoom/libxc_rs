//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta609<F: Float>(t1914: F, t40772: F, t3034: F, t336: F, t221: F, t697: F, t1016: F, t1081: F, t2752: F, t1864: F, t2241: F, t608: F, t9231: F) -> (F, F, F, F, F, F, F) {
        let (t82312, t82510, t82631, t82985, t83555, t83718, t83722) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2048::<F>(t1914, t40772, t3034, t336, t221, t697, t1016, t1081, t2752, t1864, t2241, t608, t9231);
    (t82312, t82510, t82631, t82985, t83555, t83718, t83722)
}
