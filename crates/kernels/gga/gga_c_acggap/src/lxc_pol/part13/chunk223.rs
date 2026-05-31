//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 223/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk223<F: Float>(t60: F, t803: F, t40: F, t123: F, t203: F, t84: F, t281: F, t467: F, t191: F) -> (F, F, F, F, F, F, F) {
    let t804 = t60 * t803;
    let t805 = t40 * t804;
    let t807 = t203 * t123 * t84;
    let t808 = t281 * t807;
    let t809 = F::cast_from(0.24415263074675393405e-3_f64) * t808;
    let t811 = t467 * t467;
    let t813 = t191 * t191;
    let t814 = F::cast_from(1.0_f64) / t813;
    (t804, t805, t807, t809, t811, t813, t814)
}
