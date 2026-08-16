//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 725/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk725<F: Float>(t11416: F, t11036: F, t11041: F, t11043: F, t11048: F, t11052: F, t11056: F, t11061: F, t11066: F, t11070: F, t11073: F, t11076: F, t11395: F, t11399: F, t11404: F, t11408: F, t11413: F, t7771: F, t8190: F, t8195: F) -> F {
    let t11417 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11416;
    let t11418 = t8195 / F::cast_from(18.0_f64) - t11036 / F::cast_from(27.0_f64) - t11041 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t11043 - t11048 / F::cast_from(9.0_f64) - t11052 / F::cast_from(3.0_f64) - t11056 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11061 - t7771 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11066 + t11070 - t11073 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t11076 - t8190 - t11395 / F::cast_from(6.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11399 + F::cast_from(11.0_f64) / F::cast_from(27.0_f64) * t11404 + t11408 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11413 - t11417;
    t11418
}
