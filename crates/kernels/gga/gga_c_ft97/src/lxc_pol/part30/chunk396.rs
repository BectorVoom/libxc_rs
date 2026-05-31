//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 396/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk396<F: Float>(t6386: F, t871: F, t296: F, t1901: F, t193: F, t446: F, t6272: F, t6275: F, t6280: F, t6284: F, t6289: F, t6293: F, t6298: F, t6300: F, t6304: F, t6349: F, t6355: F, t6359: F, t6362: F, t6367: F, t6371: F, t6376: F, t89: F) -> (F, F) {
    let t6387 = t871 * t6386;
    let t6388 = t296 * t6387;
    let t6391 = t6272 + t1901 * t6275 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t6280 - t446 * t6284 / F::cast_from(3.0_f64) + t446 * t6289 / F::cast_from(3.0_f64) - t446 * t6293 / F::cast_from(3.0_f64) - t6298 - t446 * t6300 / F::cast_from(9.0_f64) - t446 * t6304 / F::cast_from(3.0_f64) + t89 * t193 * t6349 / F::cast_from(3.0_f64) - t446 * t6355 / F::cast_from(3.0_f64) + t6359 + t1901 * t6362 / F::cast_from(9.0_f64) + t446 * t6367 / F::cast_from(3.0_f64) - t446 * t6371 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t6376 - t446 * t6388 / F::cast_from(3.0_f64);
    (t6388, t6391)
}
