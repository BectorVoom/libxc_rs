//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1152/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1152<F: Float>(t29372: F, t303: F, t2237: F, t27339: F, t28465: F, t28467: F, t29324: F, t29338: F, t29341: F, t29344: F, t29355: F, t29358: F, t29362: F, t29366: F, t29370: F, t7908: F, t8148: F, t8151: F, t8159: F) -> (F, F) {
    let t29373 = t303 * t29372;
    let t29377 = -F::new(0.49745833333333333332e-2) * t29338 + F::new(0.33163888888888888888e-2) * t29341 + F::new(0.69505208333333333333e-3) * t2237 * t29344 - F::new(0.37069444444444444444e-2) * t8151 * t8159 - F::new(0.37069444444444444444e-2) * t8151 * t8148 - F::new(0.185671721767578125e-4) * t27339 * t29324 - F::new(0.33163888888888888888e-2) * t29355 - F::new(0.23168402777777777778e-3) * t7908 * t29358 - F::new(0.30891203703703703704e-3) * t7908 * t29362 - F::new(0.88437037037037037034e-2) * t29366 - F::new(0.33163888888888888888e-2) * t29370 + F::new(0.24872916666666666666e-2) * t29373 + F::new(0.33163888888888888888e-2) * t28465 - F::new(0.46336805555555555556e-3) * t28467;
    (t29373, t29377)
}
