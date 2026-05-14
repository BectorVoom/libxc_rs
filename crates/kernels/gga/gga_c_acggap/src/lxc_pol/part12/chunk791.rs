//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 791/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk791<F: Float>(t1084: F, t30148: F, t30159: F, t7586: F, t377: F, t7779: F, t606: F, t7: F, t7508: F, t8: F, t151: F, t56: F, t593: F, t1994: F, t1039: F, t1997: F, t3055: F) -> (F, F, F, F, F, F, F) {
    let t30162 = t30159 * t7586 * t30148 * t1084;
    let t30169 = t377 * t7779;
    let t30170 = t30169 * t606;
    let t30174 = t7508 * t7;
    let t30176 = 1.0 / t8 / t30174;
    let t30179 = t151 * t593 * t30176 * t56;
    let t30180 = t30179 * t1994;
    let t30183 = t3055 * t1997 * t1039;
    (t30162, t30169, t30170, t30174, t30179, t30180, t30183)
}
