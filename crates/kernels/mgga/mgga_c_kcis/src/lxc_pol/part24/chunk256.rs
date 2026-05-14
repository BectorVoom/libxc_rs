//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 256/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk256<F: Float>(t1085: F, t304: F, t355: F, t360: F, t303: F, t1017: F, t86: F) -> (F, F, F, F, F) {
    let t1086 = t304 * t1085;
    let t1087 = t1086 * t355;
    let t1088 = t1087 * t360;
    let t1089 = t303 * t1088;
    let t1092 = t86 * t1017 * t304;
    (t1086, t1087, t1088, t1089, t1092)
}
