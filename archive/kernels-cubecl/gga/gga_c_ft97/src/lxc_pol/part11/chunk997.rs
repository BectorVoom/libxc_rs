//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 997/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk997<F: Float>(t8392: F, t9350: F, t9355: F, t144: F, t167: F, t1901: F, t1986: F, t2185: F, t2221: F, t2230: F, t3434: F, t3440: F, t39641: F, t40262: F, t40696: F, t40698: F, t40700: F, t40720: F, t40722: F, t446: F, t574: F, t616: F, t9007: F, t9133: F) -> F {
    let t40727 = t8392 * t9350;
    let t40729 = t8392 * t9355;
    let t40731 = F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t40696 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t40698 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t2221 * t3440 * t40700 - F::cast_from(2.0_f64) * t446 * t144 * t39641 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t616 * t9007 - t446 * t574 * t167 * t40262 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) * t446 * t2185 * t2230 * t1986 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t40720 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t9133 * t3434 * t40722 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t40727 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t40729;
    t40731
}
