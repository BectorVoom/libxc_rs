//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 778/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk778<F: Float>(t4608: F, t8392: F, t11837: F, t979: F, t83: F, t3238: F, t3255: F, t942: F, t452: F, t488: F, t4462: F, t447: F, t499: F) -> (F, F, F, F, F, F, F) {
    let t16083 = t8392 * t4608;
    let t16085 = t11837 * t979;
    let t16086 = t83 * t16085;
    let t16089 = t3238 * t3255;
    let t16090 = t83 * t16089;
    let t16093 = t942 * t3255;
    let t16095 = t452 * t488 * t16093;
    let t16099 = t447 * t499 * t4462;
    (t16083, t16085, t16086, t16089, t16090, t16095, t16099)
}
