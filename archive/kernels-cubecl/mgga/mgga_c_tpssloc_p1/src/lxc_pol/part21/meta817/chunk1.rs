//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2880/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2880<F: Float>(t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t59657: F, t60161: F, t60163: F, t60166: F, t60168: F, t60171: F, t60173: F, t60176: F) -> F {
    let t60185 = -F::cast_from(0.258925e1_f64) * t60161 + F::cast_from(0.11038e0_f64) * t60163 - F::cast_from(0.82785e-1_f64) * t60166 + F::cast_from(0.18396666666666666667e0_f64) * t60168 + F::cast_from(0.33114e0_f64) * t60171 - F::cast_from(0.91983333333333333334e-1_f64) * t60173 - F::cast_from(0.89459259259259259257e-1_f64) * t59657 + F::cast_from(0.258925e1_f64) * t60176 + F::cast_from(0.73586666666666666667e0_f64) * t48155 - F::cast_from(0.12264444444444444444e0_f64) * t48157 - F::cast_from(0.44152e0_f64) * t48159 - F::cast_from(0.22076e0_f64) * t48161 - F::cast_from(0.22076e0_f64) * t48163 + F::cast_from(0.73586666666666666666e-1_f64) * t48165 + F::cast_from(0.36793333333333333333e-1_f64) * t48167;
    t60185
}
