//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 165/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk165<F: Float>(t488: F, t492: F, t83: F, t28: F, t442: F, t446: F, t449: F, t454: F, t482: F, t89: F, t103: F, t480: F) -> (F, F, F, F) {
    let t493 = t488 * t492;
    let t494 = t83 * t493;
    let t497 = -t442 - t446 * t449 / F::cast_from(9.0_f64) - t446 * t454 / F::cast_from(3.0_f64) + t89 * t28 * t482 / F::cast_from(3.0_f64) - t446 * t494 / F::cast_from(3.0_f64);
    let t499 = t480 * t103;
    (t493, t494, t497, t499)
}
