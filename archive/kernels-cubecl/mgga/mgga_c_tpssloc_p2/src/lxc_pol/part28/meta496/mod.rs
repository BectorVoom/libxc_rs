//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1713;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta496<F: Float>(t1307: F, t26421: F, t26446: F, t26331: F, t16036: F, t550: F, t6976: F, t1992: F, t16040: F, t7696: F, t794: F, t6897: F, t12461: F, t2094: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26447, t26448, t26449, t26461, t26462, t26463, t26466, t26467, t26468, t26474, t26475) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1713::<F>(t1307, t26421, t26446, t26331, t16036, t550, t6976, t1992, t16040, t7696, t794, t6897);
        let t26558 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1714::<F>(t12461, t2094);
    (t26447, t26448, t26449, t26461, t26462, t26463, t26466, t26467, t26468, t26474, t26475, t26558)
}
