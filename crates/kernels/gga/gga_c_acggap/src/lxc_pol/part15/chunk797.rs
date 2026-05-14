//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 797/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk797<F: Float>(t30225: F, t431: F, t1966: F, t980: F, t606: F, t377: F, t7636: F, t1994: F, t30193: F, t601: F, t1973: F, t7610: F, t1985: F, t30196: F, t3668: F, t587: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30226 = t30225 * t431;
    let t30228 = t980 * t1966;
    let t30229 = t30228 * t606;
    let t30231 = t377 * t7636;
    let t30232 = t30231 * t1994;
    let t30238 = t30193 * t601;
    let t30240 = t7610 * t1973;
    let t30242 = t30196 * t1985;
    let t30244 = t587 * t3668;
    (t30226, t30228, t30229, t30231, t30232, t30238, t30240, t30242, t30244)
}
