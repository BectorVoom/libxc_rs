//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 255/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk255<F: Float>(t1072: F, t19: F, t661: F, t1068: F, t136: F, t141: F, t435: F) -> (F, F, F, F) {
    let t1074 = t1072 * t19 * t661;
    let t1075 = t1068 * t136 * t1074;
    let t1076 = t1075 / 12.0;
    let t1083 = t141 * t435;
    (t1074, t1075, t1076, t1083)
}
