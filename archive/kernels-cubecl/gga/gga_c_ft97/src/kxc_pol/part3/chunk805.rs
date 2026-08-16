//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 805/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk805<F: Float>(t11031: F, t11043: F, t11076: F, t11404: F, t11778: F, t11798: F, t16464: F, t16469: F, t16472: F, t16476: F, t8455: F, t16503: F, t16515: F, t16523: F) -> F {
    let t16531 = -t11031 - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t11043 + t11778 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11076 - t8455 + t16464 / F::cast_from(6.0_f64) + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11404 - t11798 - t16469 / F::cast_from(12.0_f64) - t16472 / F::cast_from(6.0_f64) + t16476 / F::cast_from(8.0_f64);
    let t16533 = t16503 + t16515 + t16523 + t16531;
    t16533
}
