//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1112/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1112<F: Float>(t31855: F, t32955: F, t32967: F, t36340: F, t36349: F, t36370: F, t37971: F, t37972: F, t37973: F, t37979: F, t37980: F, t37983: F, t37985: F, t40533: F, t40537: F, t40542: F, t40546: F, t40549: F) -> (F,) {
    let t42160 = 0.15724046144802076034e-2 * t40533 + 0.15724046144802076034e-2 * t40537 - t32955 + t36340 - 0.6289618457920830414e-2 * t40542 - 0.90702367218671976884e-1 * t36349 - t37971 + t37972 + t37973 + 0.34299214494455789578e-2 * t31855 + t37979 + t37980 + t40546 / 48.0 - 0.68598428988911579156e-2 * t36370 - t37983 + t32967 - t37985 + 0.85748036236139473944e-3 * t40549;
    (t42160,)
}
