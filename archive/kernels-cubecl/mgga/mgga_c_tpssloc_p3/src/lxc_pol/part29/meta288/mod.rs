//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta288<F: Float>(t207: F, t215: F, t9569: F, t2570: F, t782: F, t2573: F, t2690: F, t59: F, t154: F, t2588: F, t21: F, t795: F) -> (F, F, F, F, F, F, F) {
        let (t9572, t9573, t9574, t9577, t9579, t9580, t9583) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1314::<F>(t207, t215, t9569, t2570, t782, t2573, t2690, t59, t154, t2588, t21, t795);
    (t9572, t9573, t9574, t9577, t9579, t9580, t9583)
}
