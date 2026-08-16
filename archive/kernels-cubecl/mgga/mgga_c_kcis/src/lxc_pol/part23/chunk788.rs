//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 788/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk788<F: Float>(t1420: F, t4016: F, t4031: F, t532: F, t1401: F, t4039: F, t4142: F, t4178: F, t25: F, t4008: F, t493: F, t499: F, t737: F) -> (F, F, F, F, F, F) {
    let t12087 = t4016 * t1420;
    let t12089 = t532 * t4031;
    let t12091 = t1401 * t4039;
    let t12119 = t4142 * t4178;
    let t12124 = t25 * t4008;
    let t12125 = t493 * t12124;
    let t12127 = t737 * t499;
    (t12087, t12089, t12091, t12119, t12125, t12127)
}
