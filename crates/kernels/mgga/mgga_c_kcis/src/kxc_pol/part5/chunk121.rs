//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 121/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk121<F: Float>(t345: F, t348: F, t278: F, t344: F) -> (F, F, F) {
    let t349 = t345 * t348;
    let t352 = t278 * t278;
    let t354 = F::new(0.98556445e-3) * t344 * t349 - F::new(2.0) * t352;
    let t355 = F::new(1.0) / t354;
    (t349, t354, t355)
}
