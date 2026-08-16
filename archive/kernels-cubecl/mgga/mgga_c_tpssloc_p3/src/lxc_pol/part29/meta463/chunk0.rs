//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1789/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1789<F: Float>(t22893: F, t6639: F, t23164: F, t6546: F, t6551: F) -> (F, F, F, F) {
    let t23165 = t22893 * t6639;
    let t23166 = t23164 * t23165;
    let t23167 = F::cast_from(0.16449340668482264365e-1_f64) * t23166;
    let t23168 = t6546 * t6551;
    (t23165, t23166, t23167, t23168)
}
