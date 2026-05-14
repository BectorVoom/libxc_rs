//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 747/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk747<F: Float>(t1197: F, t19116: F, t3780: F, t4125: F, t1701: F, t17975: F, t811: F, t820: F, t19100: F, t800: F) -> (F, F, F, F, F) {
    let t19117 = t19116 * t1197;
    let t19120 = t3780 * t4125;
    let t19121 = t1701 * t19120;
    let t19125 = t1701 * t17975 * t811;
    let t19128 = t17975 * t820;
    let t19129 = t1701 * t19128;
    let t19132 = t800 * t19100;
    (t19117, t19121, t19125, t19129, t19132)
}
