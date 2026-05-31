//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 806/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk806<F: Float>(t103: F, t16533: F, t82: F, t1882: F, t4569: F, t4595: F, t11897: F, t11913: F, t11981: F, t11999: F, t12002: F, t16482: F, t16486: F, t16490: F, t1901: F, t28: F, t446: F, t8475: F, t8485: F, t8516: F, t8534: F, t89: F) -> F {
    let t16535 = t82 * t16533 * t103;
    let t16539 = t1882 * t4569;
    let t16541 = t1882 * t4595;
    let t16544 = -F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8475 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8485 - t446 * t16482 / F::cast_from(3.0_f64) - t11897 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t16486 - t11913 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8516 - t8534 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16490 + t89 * t28 * t16535 / F::cast_from(3.0_f64) - t11981 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16539 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16541 - t11999 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12002;
    t16544
}
