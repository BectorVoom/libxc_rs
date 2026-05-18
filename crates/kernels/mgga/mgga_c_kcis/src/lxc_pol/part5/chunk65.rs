//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 65/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk65<F: Float>(t157: F, t161: F, t155: F, t142: F, rho0: F, rho1: F) -> (F, F, F, F, F) {
    let t162 = t157 * t161;
    let t164 = F::new(1.0) + t155 / F::new(8.0) - t162 / F::new(64.0);
    let t165 = F::new(1.0) / t164;
    let t166 = t142 * t165;
    let t167 = rho0 - rho1;
    (t162, t164, t165, t166, t167)
}
