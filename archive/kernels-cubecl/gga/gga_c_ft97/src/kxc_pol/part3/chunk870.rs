//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 870/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk870<F: Float>(t160: F, t17486: F, t16978: F, t17062: F, t17087: F, t17106: F, t17182: F, t17356: F, t17401: F, t17410: F, t17418: F, t17500: F) -> F {
    let t17510 = t17486 * t160;
    let t17522 = F::cast_from(2.0_f64) * t17510 - F::cast_from(2.0_f64) * t17410 - F::cast_from(4.0_f64) * t17106 + F::cast_from(8.0_f64) * t17401 - F::cast_from(4.0_f64) * t17062 + F::cast_from(4.0_f64) * t17418 - F::cast_from(12.0_f64) * t17087 + F::cast_from(8.0_f64) * t17182 - F::cast_from(2.0_f64) * t17500 + F::cast_from(4.0_f64) * t16978 - F::cast_from(2.0_f64) * t17356;
    t17522
}
