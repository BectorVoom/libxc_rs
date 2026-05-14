//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 929/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk929<F: Float>(t1674: F, t8373: F, t10409: F, t1679: F, t560: F, t2354: F, t469: F, t301: F, t694: F, t11883: F, t624: F, t10956: F, t467: F, t9099: F, t33857: F, t33861: F) -> (F, F, F, F, F, F, F, F, F) {
    let t36592 = 12.0 * t1674 * t8373;
    let t36601 = 2.0 * t1679 * t10409 * t560;
    let t36602 = t2354 * t469;
    let t36605 = 6.0 * t694 * t36602 * t301;
    let t36610 = t624 * t11883;
    let t36617 = 2.0 * t1679 * t10956 * t467;
    let t36619 = 4.0 * t1679 * t9099;
    let t36823 = 0.12579236915841660827e-2 * t33857;
    let t36825 = 35.0 / 216.0 * t33861;
    (t36592, t36601, t36602, t36605, t36610, t36617, t36619, t36823, t36825)
}
