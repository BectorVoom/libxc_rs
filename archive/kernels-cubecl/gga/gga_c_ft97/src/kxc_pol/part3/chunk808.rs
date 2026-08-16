//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 808/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk808<F: Float>(t103: F, t16533: F, t16085: F, t16089: F, t16199: F, t16204: F, t16215: F, t16247: F, t16251: F, t16279: F, t16292: F, t16481: F) -> F {
    let t16550 = t16533 * t103;
    let t16562 = F::cast_from(2.0_f64) * t16550 - F::cast_from(2.0_f64) * t16247 - F::cast_from(4.0_f64) * t16085 + F::cast_from(8.0_f64) * t16279 - F::cast_from(4.0_f64) * t16089 + F::cast_from(4.0_f64) * t16251 - F::cast_from(12.0_f64) * t16199 + F::cast_from(8.0_f64) * t16204 - F::cast_from(2.0_f64) * t16215 + F::cast_from(4.0_f64) * t16292 - F::cast_from(2.0_f64) * t16481;
    t16562
}
