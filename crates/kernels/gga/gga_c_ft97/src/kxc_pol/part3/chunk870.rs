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
    let t17522 = F::new(2.0) * t17510 - F::new(2.0) * t17410 - F::new(4.0) * t17106 + F::new(8.0) * t17401 - F::new(4.0) * t17062 + F::new(4.0) * t17418 - F::new(12.0) * t17087 + F::new(8.0) * t17182 - F::new(2.0) * t17500 + F::new(4.0) * t16978 - F::new(2.0) * t17356;
    t17522
}
