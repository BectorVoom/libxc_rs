//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 83/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk83<F: Float>(t194: F, t272: F, t322: F, t170: F, t173: F) -> (F, F, F, F, F) {
    let t325 = F::new(0.469508e0) * t272 + F::new(0.4332925e0) * t194;
    let t326 = t325 * t325;
    let t327 = F::new(1.0) / t326;
    let t328 = t322 * t327;
    let t332 = f64::exp(-t170 * t173 * t328 / F::new(4.0));
    (t325, t326, t327, t328, t332)
}
