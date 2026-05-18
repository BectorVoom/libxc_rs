//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 131/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk131<F: Float>(t304: F, t339: F, t348: F, t365: F, t368: F, t86: F, t355: F) -> (F, F, F) {
    let t369 = t304 * t339;
    let t373 = F::new(0.619125e-2) * t365 * t348 - F::new(0.39796666666666666666e-1) * t86 * t368 * t369;
    let t374 = t373 * t355;
    (t369, t373, t374)
}
