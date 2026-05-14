//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 880/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk880<F: Float>(t30407: F, t30418: F, t31102: F, t513: F, t30402: F, t30409: F, t7447: F, t8637: F, t8800: F, t30219: F, t8661: F, t30543: F, t8446: F, t30934: F, t8450: F, t30937: F, t8597: F) -> (F, F, F, F, F, F, F, F) {
    let t34586 = t30407 * t30418 * t31102 * t513;
    let t34590 = t30407 * t30402 * t30409 * t513;
    let t34592 = t7447 * t8637;
    let t34609 = t7447 * t8800;
    let t34611 = t30219 * t8661;
    let t34616 = t30543 * t8446;
    let t34618 = t30934 * t8450;
    let t34620 = t30937 * t8597;
    (t34586, t34590, t34592, t34609, t34611, t34616, t34618, t34620)
}
