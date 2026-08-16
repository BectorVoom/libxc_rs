//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 217/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk217<F: Float>(t630: F, t70: F, t41: F, t342: F, t657: F, t420: F, t703: F) -> (F, F, F) {
    let t2264 = t630 * t70;
    let t2265 = t41 * t2264;
    let t2319 = t342 * t630 * t657 / F::cast_from(12.0_f64);
    let t2320 = t420 * t703;
    (t2265, t2319, t2320)
}
