//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 988/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk988<F: Float>(t2336: F, t5217: F, t89: F, t5209: F, t9725: F, t10398: F, t19243: F, t19246: F, t19249: F, t19252: F, t19255: F, t19258: F, t19261: F, t19265: F, t19269: F, t19273: F, t19276: F, t19278: F, t19283: F, t19287: F, t19292: F, t19295: F, t19298: F) -> (F, F, F) {
    let t19301 = t89 * t2336 * t5217;
    let t19304 = t89 * t9725 * t5209;
    let t19306 = -t19243 / F::cast_from(6.0_f64) + t19246 / F::cast_from(18.0_f64) - t19249 / F::cast_from(9.0_f64) + t19252 / F::cast_from(9.0_f64) - t19255 / F::cast_from(27.0_f64) - F::cast_from(5.0_f64) / F::cast_from(81.0_f64) * t19258 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t19261 + t19265 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19269 + t19273 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19276 - t19278 / F::cast_from(27.0_f64) - t10398 + t19283 / F::cast_from(3.0_f64) - t19287 / F::cast_from(18.0_f64) - t19292 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t19295 + t19298 / F::cast_from(54.0_f64) - t19301 / F::cast_from(27.0_f64) + t19304 / F::cast_from(81.0_f64);
    (t19301, t19304, t19306)
}
