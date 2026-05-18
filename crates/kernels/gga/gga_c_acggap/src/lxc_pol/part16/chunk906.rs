//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 906/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk906<F: Float>(t128: F, t576: F, t7475: F, t1108: F, t7736: F, t7770: F, t7799: F, t1170: F, t31114: F) -> (F, F, F, F) {
    let t31146 = t576 * t7475 * t128;
    let t31160 = t7736 * t1108;
    let t31168 = t7799 * t7770;
    let t31195 = t1170 * t31114;
    (t31146, t31160, t31168, t31195)
}
