//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 139/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk139<F: Float>(t35: F, t383: F, t374: F, t38: F, t62: F, t43: F, t45: F, rho0: F) -> (F, F, F, F) {
    let t384 = t383 * t35;
    let t385 = t374 * t384;
    let t388 = t38 * t62;
    let t389 = t43 * rho0;
    let t391 = F::new(1.0) / t45 / t389;
    (t384, t385, t388, t391)
}
