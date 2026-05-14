//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 80/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk80<F: Float>(t218: F, t219: F, t201: F, t34: F, t35: F, t39: F) -> (F, F, F, F) {
    let t220 = t218 * t219;
    let t221 = t201 * t220;
    let t222 = 1.0 * t221;
    let t223 = t35 * t34;
    let t224 = t223 * t39;
    (t220, t222, t223, t224)
}
