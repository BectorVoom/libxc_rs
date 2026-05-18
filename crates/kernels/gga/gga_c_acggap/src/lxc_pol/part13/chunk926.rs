//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 926/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk926<F: Float>(t31142: F, t7437: F, t128: F, t576: F, t7475: F, t1108: F, t7736: F, t1967: F, t7705: F, t1988: F, t7763: F, t7767: F) -> (F, F, F, F, F, F) {
    let t31143 = t31142 * t7437;
    let t31146 = t576 * t7475 * t128;
    let t31160 = t7736 * t1108;
    let t31162 = t1967 * t7705;
    let t31164 = t1988 * t7763;
    let t31166 = t1988 * t7767;
    (t31143, t31146, t31160, t31162, t31164, t31166)
}
