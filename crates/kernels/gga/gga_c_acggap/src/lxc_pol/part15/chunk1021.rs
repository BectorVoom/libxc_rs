//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1021/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1021<F: Float>(t1814: F, t2937: F, t406: F, t1165: F, t30856: F, t604: F, t31362: F, t9597: F, t1967: F, t9687: F, t31562: F, t38778: F, t2068: F, t38827: F, t38647: F, t7346: F, t8480: F, t8896: F) -> (F, F, F, F, F, F, F, F) {
    let t40215 = t1814 * t2937 * t406;
    let t40218 = t30856 * t1165 * t604 * t40215;
    let t40220 = t31362 * t9597;
    let t40222 = t1967 * t9687;
    let t40226 = t31562 * t1165 * t604 * t38778;
    let t40230 = t2068 * t1165 * t604 * t38827;
    let t40234 = t2068 * t1165 * t604 * t38647;
    let t40237 = t7346 * t8480 * t8896;
    (t40215, t40218, t40220, t40222, t40226, t40230, t40234, t40237)
}
