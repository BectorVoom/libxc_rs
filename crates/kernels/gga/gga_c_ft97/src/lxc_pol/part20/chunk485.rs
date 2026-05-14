//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 485/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk485<F: Float>(t6386: F, t871: F, t296: F, t1901: F, t193: F, t446: F, t6272: F, t6275: F, t6280: F, t6284: F, t6289: F, t6293: F, t6298: F, t6300: F, t6304: F, t6349: F, t6355: F, t6359: F, t6362: F, t6367: F, t6371: F, t6376: F, t89: F) -> (F, F, F) {
    let t6387 = t871 * t6386;
    let t6388 = t296 * t6387;
    let t6391 = t6272 + t1901 * t6275 / 9.0 + 2.0 / 3.0 * t446 * t6280 - t446 * t6284 / 3.0 + t446 * t6289 / 3.0 - t446 * t6293 / 3.0 - t6298 - t446 * t6300 / 9.0 - t446 * t6304 / 3.0 + t89 * t193 * t6349 / 3.0 - t446 * t6355 / 3.0 + t6359 + t1901 * t6362 / 9.0 + t446 * t6367 / 3.0 - t446 * t6371 / 3.0 + 2.0 / 3.0 * t446 * t6376 - t446 * t6388 / 3.0;
    (t6387, t6388, t6391)
}
