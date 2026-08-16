//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1792;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta535<F: Float>(t2332: F, t81442: F, t22470: F, t2358: F, t63: F, t9365: F, t2752: F, t606: F, t23020: F, t6562: F, t794: F, t22641: F, t9523: F, t22690: F, t6639: F, t1887: F, t23069: F) -> (F, F, F, F, F, F, F, F) {
        let (t81443, t81445, t81446, t81547, t81571, t81573) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1792::<F>(t2332, t81442, t22470, t2358, t63, t9365, t2752, t606, t23020, t6562, t794, t22641, t9523);
        let (t81575, t81591) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1793::<F>(t22690, t6639, t81573, t1887, t23069);
    (t81443, t81445, t81446, t81547, t81571, t81573, t81575, t81591)
}
