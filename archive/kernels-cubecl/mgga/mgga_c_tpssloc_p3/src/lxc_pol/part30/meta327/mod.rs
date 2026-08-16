//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta327<F: Float>(t1294: F, t9919: F, t2663: F, t3814: F, t9905: F, t9892: F, t3826: F, t588: F, t3684: F, t9467: F, t118: F, t1284: F) -> (F, F, F, F, F, F, F) {
        let (t12094, t12097, t12103, t12105, t12106, t12109, t12110) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1353::<F>(t1294, t9919, t2663, t3814, t9905, t9892, t3826, t588, t3684, t9467, t118, t1284);
    (t12094, t12097, t12103, t12105, t12106, t12109, t12110)
}
