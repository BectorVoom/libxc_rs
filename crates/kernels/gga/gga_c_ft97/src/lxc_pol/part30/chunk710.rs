//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 710/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk710<F: Float>(t28501: F, t2862: F, t319: F, t28506: F, t1882: F, t7116: F, t7107: F, t29285: F, t29287: F, t29290: F, t29295: F, t29299: F, t29304: F, t29309: F, t29313: F, t29317: F, t29321: F, t446: F) -> F {
    let t29325 = t2862 * t319 * t28501;
    let t29329 = t2862 * t319 * t28506;
    let t29332 = t1882 * t7116;
    let t29334 = t1882 * t7107;
    let t29336 = t29285 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t29287 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t29290 + t446 * t29295 / F::cast_from(3.0_f64) + t446 * t29299 / F::cast_from(3.0_f64) + t446 * t29304 / F::cast_from(3.0_f64) + t446 * t29309 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t29313 + t446 * t29317 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t29321 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t29325 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t29329 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t29332 - t29334 / F::cast_from(9.0_f64);
    t29336
}
