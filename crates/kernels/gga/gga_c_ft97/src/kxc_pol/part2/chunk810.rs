//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 810/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk810<F: Float>(t12170: F, t4347: F, t1263: F, t8640: F, t2951: F, t4357: F, t898: F, t2938: F, t4370: F, t904: F, t231: F, t893: F, t1270: F, t2253: F, t4372: F, t668: F) -> (F, F, F, F, F, F, F, F) {
    let t14429 = t12170 * t4347;
    let t14431 = t8640 * t1263;
    let t14434 = t898 * t4357 * t2951;
    let t14437 = t2938 * t4370;
    let t14439 = t898 * t14437 * t904;
    let t14442 = t231 * t893;
    let t14445 = t8640 * t1270;
    let t14448 = 2.0 / 3.0 * t2253 * t4372;
    let t14449 = t4370 * t668;
    (t14429, t14431, t14434, t14439, t14442, t14445, t14448, t14449)
}
