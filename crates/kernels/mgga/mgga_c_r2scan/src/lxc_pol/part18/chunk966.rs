//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 966/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk966<F: Float>(t1102: F, t3314: F, t3692: F, t11004: F, t3579: F, t3582: F, t792: F, t10997: F, t3275: F, t6967: F, t795: F, t3263: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11616 = t1102 * t3314 * t3692;
    let t11618 = t3579 * t11004;
    let t11619 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t11618;
    let t11621 = t3582 * t792;
    let t11622 = t10997 * t11621;
    let t11623 = t3275 * t11622;
    let t11624 = F::cast_from(45.0_f64) / F::cast_from(64.0_f64) * t11623;
    let t11625 = t6967 * t795;
    let t11626 = t3263 * t11625;
    (t11616, t11618, t11619, t11621, t11622, t11623, t11624, t11625, t11626)
}
