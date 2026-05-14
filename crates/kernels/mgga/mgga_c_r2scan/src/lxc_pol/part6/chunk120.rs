//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 120/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk120<F: Float>(t106: F, t288: F, t97: F, rho0: F, tau0: F) -> (F, F, F) {
    let t290 = t97 * t106 * t288;
    let t291 = pow_1_3(rho0);
    let t292 = t291 * t291;
    let t294 = 1.0 / t292 / rho0;
    let t295 = tau0 * t294;
    (t290, t292, t295)
}
