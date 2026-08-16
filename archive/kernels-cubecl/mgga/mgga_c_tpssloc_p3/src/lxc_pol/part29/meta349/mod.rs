//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta349<F: Float>(t3799: F, t3853: F, t3858: F, t12267: F, t1340: F, t3719: F, t550: F, t1995: F, t67: F, t246: F, t3734: F, t3777: F, t3802: F) -> (F, F, F, F, F, F, F, F) {
        let (t12388, t12395, t12397, t12407, t12418, t12419, t12420, t12429) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1413::<F>(t3799, t3853, t3858, t12267, t1340, t3719, t550, t1995, t67, t246, t3734, t3777, t3802);
    (t12388, t12395, t12397, t12407, t12418, t12419, t12420, t12429)
}
