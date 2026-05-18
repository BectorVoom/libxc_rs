//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 841/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk841<F: Float>(t167: F, t17076: F, t2185: F, t4724: F, t558: F, t2179: F, t574: F, t609: F, t9439: F, t144: F, t1882: F, t4730: F) -> (F, F, F, F, F) {
    let t17078 = t2185 * t167 * t17076;
    let t17081 = t4724 * t558;
    let t17083 = t574 * t2179 * t17081;
    let t17086 = t4724 * t609;
    let t17087 = t9439 * t17086;
    let t17088 = t144 * t17087;
    let t17091 = t1882 * t4730;
    (t17078, t17083, t17087, t17088, t17091)
}
