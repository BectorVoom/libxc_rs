//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 127/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk127<F: Float>(t304: F, t339: F, t348: F, t365: F, t368: F, t86: F, t355: F, sigma0: F) -> (F, F, F, F) {
    let t369 = t304 * t339;
    let t373 = F::cast_from(0.619125e-2_f64) * t365 * t348 - F::cast_from(0.39796666666666666666e-1_f64) * t86 * t368 * t369;
    let t374 = t373 * t355;
    let t375 = t374 * sigma0;
    (t369, t373, t374, t375)
}
