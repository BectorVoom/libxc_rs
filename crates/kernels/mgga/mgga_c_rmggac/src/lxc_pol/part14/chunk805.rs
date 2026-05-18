//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 805/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk805<F: Float>(t34592: F, t7441: F, t7443: F, t7446: F, t7452: F, t7459: F, t7465: F, t7471: F, t7480: F, t7486: F, t7488: F, t8563: F) -> (F, F) {
    let t38251 = -t7441 + t7443 + t7446 + t7452 + t7459 + t7465 - t7471 - t7480 + t7486 + F::new(0.38422568777328955684e-2) * t7488 + t34592;
    let t38254 = F::new(0.27274661654245341728e-1) * t8563;
    (t38251, t38254)
}
