//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 892/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk892<F: Float>(t27869: F, t446: F, t6109: F, t681: F, t6879: F, t1434: F, t6887: F, t6837: F, t713: F, t2506: F, t193: F, t2371: F) -> (F, F, F, F, F, F, F) {
    let t27870 = t446 * t27869;
    let t27873 = t6109 * t681 * t6879;
    let t27876 = t1434 * t681 * t6887;
    let t27878 = t6837 * t713;
    let t27879 = t2506 * t27878;
    let t27881 = t1434 * t193 * t27879;
    let t27882 = t2371 * t6837;
    (t27870, t27873, t27876, t27878, t27879, t27881, t27882)
}
