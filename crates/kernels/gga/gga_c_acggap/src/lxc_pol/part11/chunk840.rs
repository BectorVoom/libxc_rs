//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 840/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk840<F: Float>(t435: F, t7322: F, t7323: F, t1072: F, t372: F, t721: F, t2019: F, t2059: F, t7437: F, t128: F, t576: F, t7475: F, t1108: F, t7736: F, t1967: F, t7705: F) -> (F, F, F, F, F, F, F) {
    let t31137 = t7322 * t7323 * t435;
    let t31140 = t31137 * t1072 * t372 * t721;
    let t31142 = t2019 * t2059;
    let t31143 = t31142 * t7437;
    let t31146 = t576 * t7475 * t128;
    let t31160 = t7736 * t1108;
    let t31162 = t1967 * t7705;
    (t31137, t31140, t31142, t31143, t31146, t31160, t31162)
}
