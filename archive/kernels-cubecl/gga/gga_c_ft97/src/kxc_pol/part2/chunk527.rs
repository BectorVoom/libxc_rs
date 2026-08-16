//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 527/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk527<F: Float>(t1848: F, t1883: F, t1887: F, t1888: F, t1890: F, t1901: F, t28: F, t3115: F, t3172: F, t3177: F, t3184: F, t3190: F, t3196: F, t3201: F, t3206: F, t3210: F, t446: F, t89: F) -> F {
    let t3213 = t1901 * t3115 / F::cast_from(9.0_f64) + t1883 / F::cast_from(27.0_f64) + t89 * t28 * t3172 / F::cast_from(3.0_f64) - t3177 / F::cast_from(9.0_f64) - t1848 / F::cast_from(9.0_f64) + t1887 + t1890 / F::cast_from(9.0_f64) + t1888 / F::cast_from(9.0_f64) + t1901 * t3184 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t3190 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t3196 + t1901 * t3201 / F::cast_from(9.0_f64) + t1901 * t3206 / F::cast_from(9.0_f64) - t446 * t3210 / F::cast_from(9.0_f64);
    t3213
}
