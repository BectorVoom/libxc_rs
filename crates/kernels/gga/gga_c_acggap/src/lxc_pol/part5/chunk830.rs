//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 830/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk830<F: Float>(t39: F, t55: F, t59: F, t87: F, t2693: F, t721: F, t754: F, t2965: F, t807: F, t286: F, t688: F, t796: F) -> (F, F, F, F) {
    let t11549 = F::new(24.0) * t39 * t55 * t59 * t87;
    let t11552 = F::cast_from(0.71233333333333333332e-1_f64) * t721 * t754 * t2693;
    let t11553 = t2965 * t807;
    let t11557 = F::cast_from(0.21053605041484726346e2_f64) * t286 * t688 * t796;
    (t11549, t11552, t11553, t11557)
}
