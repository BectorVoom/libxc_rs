//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1258/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1258<F: Float>(t122219: F, t6118: F, t97181: F, t109377: F, t124003: F, t2354: F, t446: F, t4917: F, t97198: F, t193: F, t3938: F, t6109: F, t6837: F, t743: F, t31020: F, t681: F) -> (F, F, F, F, F) {
    let t124157 = t6118 * t97181 * t122219;
    let t124160 = t6118 * t109377 * t124003;
    let t124164 = t446 * t2354 * t97198 * t4917;
    let t124169 = t6109 * t193 * t743 * t6837 * t3938;
    let t124172 = t6109 * t681 * t31020;
    (t124157, t124160, t124164, t124169, t124172)
}
