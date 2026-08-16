//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 499/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk499<F: Float>(t5980: F, t6280: F, t6293: F, t6301: F, t109: F, t1368: F, t1652: F, t1602: F, t558: F, t1614: F, t552: F, t559: F) -> (F, F, F, F, F, F) {
    let t6303 = t5980 + t6280 + t6293 + t6301;
    let t6304 = t6303 * t109;
    let t6308 = t1368 * t1652;
    let t6311 = t1602 * t558;
    let t6315 = t552 * t1614;
    let t6327 = t559 * t1614;
    (t6303, t6304, t6308, t6311, t6315, t6327)
}
