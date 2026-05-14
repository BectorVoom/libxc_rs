//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 870/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk870<F: Float>(t1647: F, t5942: F, t2210: F, t558: F, t5968: F, t574: F, t605: F, t1882: F, t5949: F, t5958: F, t5842: F, t609: F, t1359: F, t2157: F, t2142: F, t5869: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23416 = t5942 * t1647;
    let t23417 = t2210 * t23416;
    let t23420 = t5968 * t558;
    let t23422 = t574 * t605 * t23420;
    let t23425 = t1882 * t5949;
    let t23427 = t1882 * t5958;
    let t23429 = t5842 * t609;
    let t23431 = t574 * t605 * t23429;
    let t23434 = t1359 * t2157;
    let t23436 = t574 * t605 * t23434;
    let t23440 = t574 * t2142 * t5869;
    (t23416, t23417, t23420, t23422, t23425, t23427, t23429, t23431, t23434, t23436, t23440)
}
