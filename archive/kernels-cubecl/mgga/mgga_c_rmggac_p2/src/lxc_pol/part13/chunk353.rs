//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 353/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk353<F: Float>(t2075: F, t2103: F, t262: F, t265: F, t655: F, t2069: F, t851: F, t2074: F, t854: F, t344: F, t22: F) -> (F, F, F, F, F, F, F) {
    let t2104 = t2103 * t2075;
    let t2106 = t262 * t265;
    let t2107 = t655 * t2106;
    let t2109 = t851 * t2069;
    let t2111 = t854 * t2074;
    let t2113 = t344 * t265;
    let t2115 = t854 * t22;
    (t2104, t2106, t2107, t2109, t2111, t2113, t2115)
}
