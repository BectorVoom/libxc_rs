//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 939/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk939<F: Float>(t1208: F, t218: F, t811: F, t25057: F, t820: F, t1196: F, t1472: F, t27720: F, t2691: F, t7005: F) -> (F, F, F, F, F, F, F) {
    let t28628 = t218 * t1208;
    let t28629 = t28628 * t811;
    let t28630 = t25057 * t28629;
    let t28633 = t28628 * t820;
    let t28634 = t25057 * t28633;
    let t28637 = t218 * t1196;
    let t28638 = t28637 * t820;
    let t28639 = t25057 * t28638;
    let t28646 = t1472 * t27720;
    let t28652 = t2691 * t7005;
    (t28628, t28630, t28634, t28637, t28639, t28646, t28652)
}
