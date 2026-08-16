//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1858;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta482<F: Float>(t3196: F, t6800: F, t6799: F, t23602: F, t3127: F, t1011: F, t3131: F) -> (F, F, F, F) {
        let (t23673, t23674, t23677, t23678) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1858::<F>(t3196, t6800, t6799, t23602, t3127, t1011, t3131);
    (t23673, t23674, t23677, t23678)
}
