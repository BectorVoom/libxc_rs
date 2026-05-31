//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 153/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk153<F: Float>(t13: F, t171: F, t70: F, t170: F, t180: F, t11: F, t172: F) -> (F, F, F, F) {
    let t625 = F::cast_from(1.0_f64) / t171 / t13;
    let t626 = t625 * t70;
    let t629 = t170 * t626 * t180 / F::cast_from(6.0_f64);
    let t630 = t11 * t172;
    (t625, t626, t629, t630)
}
