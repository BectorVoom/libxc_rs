//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 815/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk815<F: Float>(t21953: F, t89: F, t9716: F, t10398: F, t14715: F, t14895: F, t19246: F, t19249: F, t19298: F, t19301: F, t19304: F, t21947: F, t21951: F) -> (F, F) {
    let t21955 = t89 * t9716 * t21953;
    let t21957 = t19246 / F::cast_from(6.0_f64) - t19249 / F::cast_from(3.0_f64) + t19298 / F::cast_from(18.0_f64) - t19301 / F::cast_from(9.0_f64) + t19304 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14895 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14715 - t21947 / F::cast_from(3.0_f64) - t21951 / F::cast_from(3.0_f64) - t10398 - F::cast_from(5.0_f64) / F::cast_from(81.0_f64) * t21955;
    (t21955, t21957)
}
