//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 91/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk91<F: Float>(t195: F, t198: F, t222: F, t251: F, t258: F, t266: F, t273: F, t4: F, t71: F, t84: F) -> F {
    let t276 = F::cast_from(0.53237641966666666666e-3_f64) * t4 * t195 * t71 + F::new(1.0) * t251 * t258 - t198 - t222 + F::cast_from(0.18311447306006545054e-3_f64) * t4 * t195 * t84 + F::cast_from(0.5848223622634646207e0_f64) * t266 * t273;
    t276
}
