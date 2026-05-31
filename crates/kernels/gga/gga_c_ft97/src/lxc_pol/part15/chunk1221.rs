//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1221/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1221<F: Float>(t299: F, t91432: F, t91469: F, t13: F, t20494: F, t21100: F, t21794: F, t22488: F, t86576: F, t88053: F, t89765: F) -> F {
    let t300 = F::cast_from(10000000.0_f64) <= t299;
    let t91471 = piecewise3::<F>(t300, F::cast_from(0.0_f64), t91432 + t91469);
    let tv4rho44 = F::cast_from(4.0_f64) * t20494 + F::cast_from(4.0_f64) * t21100 + F::cast_from(4.0_f64) * t21794 + F::cast_from(4.0_f64) * t22488 + t13 * (t86576 + t88053 + t89765 + t91471);
    tv4rho44
}
