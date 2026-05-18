//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 262/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk262<F: Float>(t106: F, t795: F, t797: F, t97: F, t292: F, t415: F, rho0: F, tau0: F) -> (F, F, F) {
    let t799 = t97 * t106 * t795 * t797;
    let t800 = rho0 * rho0;
    let t802 = F::new(1.0) / t292 / t800;
    let t803 = tau0 * t802;
    let t806 = t415 / F::new(2.0);
    (t799, t803, t806)
}
