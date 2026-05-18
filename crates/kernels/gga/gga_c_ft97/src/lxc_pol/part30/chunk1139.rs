//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1139/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1139<F: Float>(t152826: F, t193: F, t2781: F, t6308: F, t143329: F, t143333: F, t143336: F, t143339: F, t143355: F, t143366: F, t143371: F, t153435: F, t153439: F, t153443: F, t153449: F, t153453: F, t153456: F, t153460: F, t153464: F) -> (F, F) {
    let t153468 = t6308 * t193 * t2781 * t152826;
    let t153470 = -F::new(2.0) * t153435 - t153439 / F::new(6.0) - t153443 / F::new(9.0) + F::new(2.0) / F::new(27.0) * t143329 + t143333 - t143336 + t143339 / F::new(9.0) - t143355 / F::new(36.0) + t153449 / F::new(3.0) - t143366 - t143371 / F::new(27.0) + F::new(2.0) / F::new(27.0) * t153453 - F::new(2.0) / F::new(9.0) * t153456 + F::new(4.0) / F::new(3.0) * t153460 - F::new(2.0) / F::new(3.0) * t153464 - t153468 / F::new(6.0);
    (t153468, t153470)
}
