//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 928/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk928<F: Float>(t2132: F, t2331: F, t7885: F, t864: F, t2333: F, t848: F, t2342: F, t30005: F, t2131: F, t847: F, t7994: F, t8998: F, t32041: F, t36019: F, t7932: F, t694: F, t8379: F) -> (F, F, F, F, F, F, F) {
    let t36526 = t7885 * t2132 * t2331 * t864;
    let t36531 = t848 * t2333;
    let t36533 = t30005 * t2342;
    let t36541 = t2131 * t2132 * t2331 * t847;
    let t36543 = t8998 * t7994;
    let t36555 = t32041 * t7932 * t36019;
    let t36575 = 6.0 * t694 * t8379;
    (t36526, t36531, t36533, t36541, t36543, t36555, t36575)
}
