//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 69/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk69<F: Float>(t157: F, t161: F, t155: F, rho0: F, rho1: F) -> (F, F, F, F) {
    let t162 = t157 * t161;
    let t164 = F::cast_from(1.0_f64) + t155 / F::cast_from(8.0_f64) - t162 / F::cast_from(64.0_f64);
    let t165 = F::cast_from(1.0_f64) / t164;
    let t167 = rho0 - rho1;
    (t162, t164, t165, t167)
}
