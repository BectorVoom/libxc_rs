//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 895/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk895<F: Float>(t7538: F, t7720: F, t7724: F, t30228: F, t601: F, t30174: F, t151: F, t56: F, t593: F, t606: F, t30225: F, t425: F) -> (F, F, F, F, F, F) {
    let t30655 = t7538 * t7720;
    let t30657 = t7538 * t7724;
    let t30663 = t30228 * t601;
    let t30665 = F::cast_from(1.0_f64) / t30174;
    let t30668 = t151 * t593 * t30665 * t56;
    let t30669 = t30668 * t601;
    let t30671 = t30668 * t606;
    let t30673 = t30225 * t425;
    (t30655, t30657, t30663, t30669, t30671, t30673)
}
