//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta559<F: Float>(t52: F, t8027: F, t2136: F, t461: F, t7573: F, t7324: F, t3448: F, t4729: F, t475: F, t5011: F, t68: F, t7328: F) -> (F, F, F, F, F, F) {
        let (t27681, t27683, t27684, t27687, t27691, t27692) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1962::<F>(t52, t8027, t2136, t461, t7573, t7324, t3448, t4729, t475, t5011, t68, t7328);
    (t27681, t27683, t27684, t27687, t27691, t27692)
}
