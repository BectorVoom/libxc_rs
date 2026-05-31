//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 848/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk848<F: Float>(t17189: F, t2222: F, t2221: F, t17151: F, t17155: F, t17158: F, t17161: F, t17165: F, t17170: F, t17174: F, t17178: F, t17183: F, t17186: F, t1901: F, t446: F, t9270: F, t9272: F, t9298: F, t9321: F) -> F {
    let t17190 = t2222 * t17189;
    let t17191 = t2221 * t17190;
    let t17194 = -F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9270 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9272 - F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t9298 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t17151 - t446 * t17155 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t17158 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t17161 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t17165 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9321 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t17170 - t446 * t17174 / F::cast_from(3.0_f64) - t446 * t17178 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t17183 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t17186 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t17191;
    t17194
}
