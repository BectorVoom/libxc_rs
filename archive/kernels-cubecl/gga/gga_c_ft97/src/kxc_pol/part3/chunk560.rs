//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 560/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk560<F: Float>(t370: F, t4495: F, t27: F, t89: F, t1545: F, t3161: F, t3166: F, t4420: F, t4424: F, t4428: F, t4434: F, t4439: F) -> (F, F, F) {
    let t4496 = t370 * t4495;
    let t4498 = t89 * t27 * t4496;
    let t4500 = t1545 + t3161 + t3166 - t4420 / F::cast_from(27.0_f64) + t4424 / F::cast_from(9.0_f64) + t4428 / F::cast_from(9.0_f64) - t4434 / F::cast_from(18.0_f64) + t4439 / F::cast_from(3.0_f64) - t4498 / F::cast_from(6.0_f64);
    (t4496, t4498, t4500)
}
