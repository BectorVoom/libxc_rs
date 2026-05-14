//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 427/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk427<F: Float>(t372: F, t604: F, t142: F, t2060: F, t592: F, t595: F) -> (F, F, F, F) {
    let t2061 = t604 * t372;
    let t2062 = t142 * t2061;
    let t2063 = t2060 * t2062;
    let t2065 = t592 * t595;
    (t2061, t2062, t2063, t2065)
}
