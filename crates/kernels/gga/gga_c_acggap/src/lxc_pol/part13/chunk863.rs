//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 863/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk863<F: Float>(t1084: F, t30148: F, t30159: F, t7586: F, t377: F, t7779: F, t606: F, t7: F, t7508: F, t8: F, t151: F, t56: F, t593: F) -> (F, F, F, F, F) {
    let t30162 = t30159 * t7586 * t30148 * t1084;
    let t30169 = t377 * t7779;
    let t30170 = t30169 * t606;
    let t30171 = F::cast_from(0.19812298142450615803e-1_f64) * t30170;
    let t30174 = t7508 * t7;
    let t30176 = F::new(1.0) / t8 / t30174;
    let t30179 = t151 * t593 * t30176 * t56;
    (t30162, t30169, t30171, t30174, t30179)
}
