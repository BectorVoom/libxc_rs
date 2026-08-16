//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 169/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk169<F: Float>(t150: F, t198: F, t222: F, t226: F, t231: F, t278: F, t285: F, t290: F, t485: F, t487: F, t402: F, t495: F) -> (F, F) {
    let t519 = (t198 + t222 - t226 - t231 + t485 + t278 + t487 - t285 - t290) * t150;
    let t521 = t402 * t495;
    (t519, t521)
}
