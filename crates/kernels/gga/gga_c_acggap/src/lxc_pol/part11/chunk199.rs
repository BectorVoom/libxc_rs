//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 199/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk199<F: Float>(t598: F, t606: F, t572: F, t581: F, t585: F, t590: F, t602: F) -> (F,) {
    let t607 = t598 * t606;
    let t609 = t572 / 96.0 + t581 / 384.0 - 0.38203125e-2 * t585 + 0.42874018118069736972e-3 * t590 + 0.10718504529517434243e-3 * t602 - 0.15724046144802076034e-3 * t607;
    (t609,)
}
