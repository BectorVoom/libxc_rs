//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 283/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk283<F: Float>(t1176: F, t375: F, t1130: F, t1133: F, t376: F, t1085: F, t355: F, t381: F, t389: F, t1095: F) -> (F, F, F, F, F, F, F) {
    let t1177 = t375 * t1176;
    let t1179 = t1130 * t1133;
    let t1180 = t376 * t1179;
    let t1181 = t375 * t1180;
    let t1183 = t1085 * t355;
    let t1184 = t1183 * t381;
    let t1185 = t1184 * t389;
    let t1187 = t1095 * t381;
    (t1177, t1180, t1181, t1183, t1184, t1185, t1187)
}
