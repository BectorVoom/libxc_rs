//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 853/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk853<F: Float>(t10398: F, t19243: F, t19246: F, t19249: F, t19252: F, t19255: F, t19258: F, t19261: F, t19265: F, t19269: F, t19273: F, t19276: F, t19278: F, t19283: F, t19287: F, t19292: F, t19295: F, t19298: F, t19301: F, t19304: F) -> (F,) {
    let t19306 = -t19243 / 6.0 + t19246 / 18.0 - t19249 / 9.0 + t19252 / 9.0 - t19255 / 27.0 - 5.0 / 81.0 * t19258 + 4.0 / 27.0 * t19261 + t19265 / 9.0 - 2.0 / 9.0 * t19269 + t19273 / 27.0 + 2.0 / 9.0 * t19276 - t19278 / 27.0 - t10398 + t19283 / 3.0 - t19287 / 18.0 - t19292 + 2.0 / 3.0 * t19295 + t19298 / 54.0 - t19301 / 27.0 + t19304 / 81.0;
    (t19306,)
}
