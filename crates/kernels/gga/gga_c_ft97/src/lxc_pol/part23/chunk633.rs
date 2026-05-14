//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 633/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk633<F: Float>(t13780: F, t13794: F, t13809: F, t13811: F, t4354: F, t8675: F, t2253: F, t4359: F, t12170: F, t4347: F, t1263: F, t8640: F, t1270: F, t4372: F, t4339: F, t4343: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14336 = t13780 / 27.0;
    let t14341 = 2.0 / 81.0 * t13794;
    let t14346 = t13809 / 27.0;
    let t14347 = 2.0 / 27.0 * t13811;
    let t14421 = 4.0 / 9.0 * t8675 * t4354;
    let t14423 = 2.0 * t2253 * t4359;
    let t14429 = t12170 * t4347;
    let t14431 = t8640 * t1263;
    let t14445 = t8640 * t1270;
    let t14448 = 2.0 / 3.0 * t2253 * t4372;
    let t14478 = 4.0 / 9.0 * t8675 * t4339;
    let t14480 = 4.0 / 9.0 * t8675 * t4343;
    (t14336, t14341, t14346, t14347, t14421, t14423, t14429, t14431, t14445, t14448, t14478, t14480)
}
