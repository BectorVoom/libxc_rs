//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 279/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk279<F: Float>(t157: F, t360: F, t372: F, t119: F, t441: F, t186: F, t447: F) -> (F, F, F, F) {
    let t1182 = t157 * t360;
    let t1188 = t157 * t372;
    let t1215 = t119 * t441;
    let t1219 = 1.0 / t447 / t186;
    (t1182, t1188, t1215, t1219)
}
