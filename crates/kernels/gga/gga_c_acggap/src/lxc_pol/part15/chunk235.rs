//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 235/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk235<F: Float>(t286: F, t912: F, t420: F, t94: F, t377: F, t396: F) -> (F, F, F) {
    let t913 = t286 * t912;
    let t914 = F::cast_from(0.11696447245269292414e1_f64) * t913;
    let t921 = t94 * t420;
    let t935 = t377 * t396;
    (t914, t921, t935)
}
