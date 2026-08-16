//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta127 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk712;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta127<F: Float>(t290: F, t2793: F, t2842: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F, t919: F, t923: F, t307: F, t922: F, t302: F, t931: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2843, t2844, t2845, t2847, t2848, t2853, t2856, t2859) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk712::<F>(t290, t2793, t2842, t2764, t2766, t2773, t2778, t2782, t919, t923, t307, t922);
        let (t2860, t2861, t2862) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk713::<F>(t2859, t302, t931);
    (t2843, t2844, t2845, t2847, t2848, t2853, t2856, t2860, t2861, t2862)
}
