//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1161/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1161<F: Float>(t143329: F, t143339: F, t143355: F, t143371: F, t143518: F, t143519: F, t143528: F, t153435: F, t153439: F, t153443: F, t153449: F, t153453: F, t153456: F, t153460: F, t153464: F, t153468: F) -> F {
    let t154217 = -F::new(6.0) * t153435 - t153439 / F::new(2.0) - t153443 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t143329 + t143518 - t143519 + t143339 / F::new(3.0) - t143355 / F::new(12.0) + t153449 - t143528 - t143371 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t153453 - F::new(2.0) / F::new(3.0) * t153456 + F::new(4.0) * t153460 - F::new(2.0) * t153464 - t153468 / F::new(2.0);
    t154217
}
