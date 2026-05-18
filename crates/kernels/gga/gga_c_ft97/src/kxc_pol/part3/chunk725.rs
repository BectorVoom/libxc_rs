//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 725/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk725<F: Float>(t2253: F, t4359: F, t12170: F, t4347: F, t1263: F, t8640: F, t1270: F, t4372: F, t4339: F, t8675: F, t4343: F, t4335: F) -> (F, F, F, F, F, F, F, F) {
    let t14423 = F::new(2.0) * t2253 * t4359;
    let t14429 = t12170 * t4347;
    let t14431 = t8640 * t1263;
    let t14445 = t8640 * t1270;
    let t14448 = F::new(2.0) / F::new(3.0) * t2253 * t4372;
    let t14478 = F::new(4.0) / F::new(9.0) * t8675 * t4339;
    let t14480 = F::new(4.0) / F::new(9.0) * t8675 * t4343;
    let t14482 = F::new(2.0) / F::new(27.0) * t8675 * t4335;
    (t14423, t14429, t14431, t14445, t14448, t14478, t14480, t14482)
}
