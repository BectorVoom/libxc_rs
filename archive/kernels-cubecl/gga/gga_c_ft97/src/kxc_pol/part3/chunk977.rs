//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 977/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk977<F: Float>(t1701: F, t19120: F, t17975: F, t811: F, t820: F, t19100: F, t800: F, t19106: F, t285: F, t4089: F, t4092: F, t4061: F, t5261: F) -> (F, F, F, F, F, F, F) {
    let t19121 = t1701 * t19120;
    let t19125 = t1701 * t17975 * t811;
    let t19128 = t17975 * t820;
    let t19129 = t1701 * t19128;
    let t19132 = t800 * t19100;
    let t19135 = t285 * t19106;
    let t19144 = t4092 * t4089;
    let t19147 = t4061 * t5261;
    (t19121, t19125, t19129, t19132, t19135, t19144, t19147)
}
